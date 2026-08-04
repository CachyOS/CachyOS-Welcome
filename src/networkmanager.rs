//! `NetworkManager` D-Bus client for changing a connection's DNS settings.

use std::collections::HashMap;

use anyhow::{Context, Result};
use tokio::runtime::Runtime;
use zbus::Connection;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

type Settings = HashMap<String, HashMap<String, OwnedValue>>;

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager.Settings",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings"
)]
trait Settings {
    fn list_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager.Settings.Connection",
    default_service = "org.freedesktop.NetworkManager"
)]
trait SettingsConnection {
    /// Full settings map (no secrets).
    fn get_settings(&self) -> zbus::Result<Settings>;

    /// Replace the connection with `properties` and save to disk.
    #[zbus(allow_interactive_auth)]
    fn update(&self, properties: &Settings) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    #[zbus(property)]
    fn active_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager.Connection.Active",
    default_service = "org.freedesktop.NetworkManager"
)]
trait ActiveConnection {
    #[zbus(property, name = "Connection")]
    fn connection_path(&self) -> zbus::Result<OwnedObjectPath>;
    #[zbus(property)]
    fn devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager.Device",
    default_service = "org.freedesktop.NetworkManager"
)]
trait Device {
    /// Reapply `connection` to the running device.
    #[zbus(allow_interactive_auth)]
    fn reapply(&self, connection: &Settings, version_id: u64, flags: u32) -> zbus::Result<()>;
}

/// `connection.dns-over-tls` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NmDnsOverTls {
    Default = -1,
    No = 0,
    // NOTE: not currently set by the UI
    #[allow(dead_code)]
    Opportunistic = 1,
    Yes = 2,
}

/// DNS changes to apply.
#[derive(Default)]
pub struct DnsMods {
    /// `ipv4.dns-data`; entries may carry a `#hostname` `DoT` SNI suffix.
    pub ipv4_dns: Option<Vec<String>>,
    /// `ipv6.dns-data`; entries may carry a `#hostname` `DoT` SNI suffix.
    pub ipv6_dns: Option<Vec<String>>,
    pub ipv4_dns_priority: Option<i32>,
    pub ipv6_dns_priority: Option<i32>,
    pub ipv4_ignore_auto_dns: Option<bool>,
    pub ipv6_ignore_auto_dns: Option<bool>,
    pub dns_over_tls: Option<NmDnsOverTls>,
}

/// Outcome of [`modify_connection_dns`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyStatus {
    /// Saved and reapplied to the live connection.
    Applied,
    /// Saved to disk, but no active device was reapplied.
    PendingReconnect,
}

/// Apply `mods` to the DNS settings of the connection.
pub fn modify_connection_dns(conn_name: &str, mods: &DnsMods) -> Result<ApplyStatus> {
    let rt = Runtime::new().context("Failed to initialize tokio runtime")?;
    rt.block_on(async {
        let conn = Connection::system().await?;
        let settings = SettingsProxy::new(&conn).await?;

        // The finder already fetched the matched connection's settings.
        let (conn_path, mut current) = find_connection_by_name(&conn, &settings, conn_name)
            .await?
            .with_context(|| format!("Network connection '{conn_name}' not found"))?;

        let connection_proxy =
            SettingsConnectionProxy::builder(&conn).path(conn_path.clone())?.build().await?;

        // Update replaces the whole connection, so patch the fetched map in place.
        apply_dns_patches(&mut current, mods);
        connection_proxy.update(&current).await?;

        // Reapply so the change is live now.
        let status = match reapply_active(&conn, &conn_path, &current).await {
            Ok(true) => ApplyStatus::Applied,
            Ok(false) => ApplyStatus::PendingReconnect,
            Err(e) => {
                tracing::warn!("Failed to reapply connection (applies on next connect): {e}");
                ApplyStatus::PendingReconnect
            },
        };
        Ok(status)
    })
}

async fn find_connection_by_name(
    conn: &Connection,
    settings: &SettingsProxy<'_>,
    name: &str,
) -> Result<Option<(OwnedObjectPath, Settings)>> {
    for path in settings.list_connections().await? {
        let proxy = SettingsConnectionProxy::builder(conn).path(&path)?.build().await?;
        let Ok(cfg) = proxy.get_settings().await else {
            continue;
        };
        let conn_id = cfg.get("connection").and_then(|group| group.get("id"));
        if conn_id.and_then(|val| <&str>::try_from(&**val).ok()) == Some(name) {
            return Ok(Some((path, cfg)));
        }
    }
    Ok(None)
}

fn set(settings: &mut Settings, group: &str, key: &str, val: Value<'static>) {
    let owned_val = OwnedValue::try_from(val).expect("failed conversion to OwnedValue");
    settings.entry(group.to_owned()).or_default().insert(key.to_owned(), owned_val);
}

/// Whether the family's `method` permits DNS settings.
fn dns_allowed(settings: &Settings, family: &str) -> bool {
    let method = settings.get(family).and_then(|g| g.get("method"));
    !matches!(
        method.and_then(|v| <&str>::try_from(&**v).ok()),
        Some("disabled" | "ignore" | "link-local" | "shared")
    )
}

/// Patch one address family's DNS keys.
fn patch_family(
    settings: &mut Settings,
    family: &str,
    dns: &Option<Vec<String>>,
    priority: Option<i32>,
    ignore_auto: Option<bool>,
) {
    if !dns_allowed(settings, family) {
        tracing::debug!("skipping {family} DNS: NM disallows it for this method");
        return;
    }
    if let Some(dns) = dns {
        // drop legacy binary key
        settings.entry(family.to_owned()).or_default().remove("dns");
        set(settings, family, "dns-data", Value::from(dns.clone()));
    }
    if let Some(priority) = priority {
        set(settings, family, "dns-priority", Value::from(priority));
    }
    if let Some(ignore) = ignore_auto {
        set(settings, family, "ignore-auto-dns", Value::from(ignore));
    }
}

/// Patch the DNS keys of `settings` in place from `mods`.
fn apply_dns_patches(settings: &mut Settings, mods: &DnsMods) {
    patch_family(
        settings,
        "ipv4",
        &mods.ipv4_dns,
        mods.ipv4_dns_priority,
        mods.ipv4_ignore_auto_dns,
    );
    patch_family(
        settings,
        "ipv6",
        &mods.ipv6_dns,
        mods.ipv6_dns_priority,
        mods.ipv6_ignore_auto_dns,
    );
    if let Some(dot) = mods.dns_over_tls {
        set(settings, "connection", "dns-over-tls", Value::from(dot as i32));
    }
}

/// Reapply settings to every device of the active connection.
async fn reapply_active(
    conn: &Connection,
    conn_path: &OwnedObjectPath,
    dict: &Settings,
) -> Result<bool> {
    let nm = NetworkManagerProxy::new(conn).await?;
    let mut applied = false;
    for active_path in nm.active_connections().await? {
        let active = ActiveConnectionProxy::builder(conn).path(&active_path)?.build().await?;
        if active.connection_path().await.ok().as_deref() != Some(&**conn_path) {
            continue;
        }
        for device_path in active.devices().await? {
            let device = DeviceProxy::builder(conn).path(&device_path)?.build().await?;
            device.reapply(dict, 0, 0).await?;
            applied = true;
        }
    }
    Ok(applied)
}

#[cfg(test)]
mod test {
    use super::*;
    use zbus::zvariant::{Array, Value};

    fn ov(val: Value<'static>) -> OwnedValue {
        OwnedValue::try_from(val).unwrap()
    }

    fn wifi_settings() -> Settings {
        let mut connection = HashMap::new();
        connection.insert("id".to_owned(), ov("MyWiFi".into()));
        connection.insert("type".to_owned(), ov("802-11-wireless".into()));
        let mut ipv4 = HashMap::new();
        ipv4.insert("method".to_owned(), ov("auto".into()));
        ipv4.insert("dns".to_owned(), ov(Value::from(Array::from(&[1u32, 1u32][..]))));
        let mut map = HashMap::new();
        map.insert("connection".to_owned(), connection);
        map.insert("ipv4".to_owned(), ipv4);
        map.insert("802-11-wireless".to_owned(), HashMap::new());
        map
    }

    fn dns_data(settings: &Settings, group: &str) -> Vec<String> {
        let arr = Array::try_from(&**settings[group].get("dns-data").unwrap()).unwrap();
        arr.iter().map(|v| <&str>::try_from(v).unwrap().to_owned()).collect()
    }

    fn int(settings: &Settings, group: &str, key: &str) -> Option<i32> {
        settings.get(group)?.get(key).and_then(|v| i32::try_from(&**v).ok())
    }

    fn text(settings: &Settings, group: &str, key: &str) -> Option<String> {
        settings.get(group)?.get(key).and_then(|v| <&str>::try_from(&**v).ok().map(String::from))
    }

    #[test]
    fn sets_dns_data() {
        let mut settings = wifi_settings();
        let mods = DnsMods {
            ipv4_dns: Some(vec!["1.1.1.1".into(), "1.0.0.1".into()]),
            ipv4_dns_priority: Some(-1),
            dns_over_tls: Some(NmDnsOverTls::Yes),
            ..Default::default()
        };
        apply_dns_patches(&mut settings, &mods);

        assert!(!settings["ipv4"].contains_key("dns"));
        assert_eq!(dns_data(&settings, "ipv4"), ["1.1.1.1", "1.0.0.1"]);
        assert_eq!(int(&settings, "ipv4", "dns-priority"), Some(-1));
        assert_eq!(int(&settings, "connection", "dns-over-tls"), Some(2));
        assert_eq!(text(&settings, "ipv4", "method").as_deref(), Some("auto"));
        assert_eq!(text(&settings, "connection", "id").as_deref(), Some("MyWiFi"));
        assert!(settings.contains_key("802-11-wireless"));
    }

    #[test]
    fn keeps_dot_hostname() {
        let mut settings = wifi_settings();
        let mods = DnsMods {
            ipv4_dns: Some(vec!["1.1.1.1#cloudflare-dns.com".into()]),
            ..Default::default()
        };
        apply_dns_patches(&mut settings, &mods);
        assert_eq!(dns_data(&settings, "ipv4"), ["1.1.1.1#cloudflare-dns.com"]);
    }

    #[test]
    fn clears_dns() {
        let mut settings = wifi_settings();
        let mods = DnsMods {
            ipv4_dns: Some(Vec::new()),
            ipv4_dns_priority: Some(0),
            dns_over_tls: Some(NmDnsOverTls::Default),
            ..Default::default()
        };
        apply_dns_patches(&mut settings, &mods);

        assert!(!settings["ipv4"].contains_key("dns"));
        assert!(dns_data(&settings, "ipv4").is_empty());
        assert_eq!(int(&settings, "ipv4", "dns-priority"), Some(0));
        assert_eq!(int(&settings, "connection", "dns-over-tls"), Some(-1));
    }

    #[test]
    fn skips_disabled_family() {
        let mut settings = wifi_settings();
        settings.insert(
            "ipv6".to_owned(),
            HashMap::from([("method".to_owned(), ov("disabled".into()))]),
        );
        let mods = DnsMods {
            ipv4_dns: Some(vec!["1.1.1.1".into()]),
            ipv6_dns: Some(vec!["2606:4700:4700::1111".into()]),
            ..Default::default()
        };
        apply_dns_patches(&mut settings, &mods);

        assert_eq!(dns_data(&settings, "ipv4"), ["1.1.1.1"]);
        assert!(!settings["ipv6"].contains_key("dns-data"));
    }

    #[test]
    fn only_touches_requested_fields() {
        let mut settings = wifi_settings();
        let mods = DnsMods { dns_over_tls: Some(NmDnsOverTls::Yes), ..Default::default() };
        apply_dns_patches(&mut settings, &mods);

        let ipv4 = &settings["ipv4"];
        assert!(ipv4.contains_key("dns"));
        assert!(!ipv4.contains_key("dns-data"));
        assert!(!ipv4.contains_key("dns-priority"));
        assert_eq!(int(&settings, "connection", "dns-over-tls"), Some(2));
    }
}
