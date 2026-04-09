use std::sync::{LazyLock, Mutex};

use tokio::runtime::Runtime;
use tracing::error;
use zbus::proxy::MethodFlags;
use zbus::zvariant;

/// Whether to operate on the system-wide or per-user systemd instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    System,
    User,
}

static MANAGER: LazyLock<Mutex<SystemdUnitManager>> =
    LazyLock::new(|| Mutex::new(SystemdUnitManager::new()));

/// Little helper to manage on unit caches
#[derive(Debug, Clone)]
struct SystemdUnitManager {
    system_units: Vec<String>,
    user_units: Vec<String>,
}

impl SystemdUnitManager {
    fn new() -> Self {
        Self { system_units: Vec::new(), user_units: Vec::new() }
    }

    /// Refreshes system-level units
    fn refresh_system(&mut self) {
        let rt = Runtime::new().expect("Failed to initialize tokio runtime");
        match rt.block_on(get_enabled_system_units()) {
            Ok(units) => self.system_units = units,
            Err(e) => error!("Failed to load system units: {e}"),
        }
    }

    /// Refreshes user-level units
    fn refresh_user(&mut self) {
        let rt = Runtime::new().expect("Failed to initialize tokio runtime");
        match rt.block_on(get_enabled_user_units()) {
            Ok(units) => self.user_units = units,
            Err(e) => error!("Failed to load user units: {e}"),
        }
    }

    /// Checks if units are enabled in System scope
    fn system_enabled(&self, units_str: &str) -> bool {
        units_str.split_whitespace().all(|unit| self.system_units.contains(&unit.to_string()))
    }

    /// Checks if units are enabled in User scope
    fn user_enabled(&self, units_str: &str) -> bool {
        units_str.split_whitespace().all(|unit| self.user_units.contains(&unit.to_string()))
    }

    /// Checks if any units enabled
    fn any_enabled(&self, units_str: &str) -> bool {
        units_str.split_whitespace().all(|unit| {
            self.system_units.contains(&unit.to_string())
                || self.user_units.contains(&unit.to_string())
        })
    }
}

/// Filters systemd services and timers by enabled between reboots status
async fn get_enabled_units(conn: &zbus::Connection) -> anyhow::Result<Vec<String>> {
    let manager = zbus_systemd::systemd1::ManagerProxy::new(conn).await?;
    let services = manager
        .list_unit_files_by_patterns(vec!["enabled".into()], vec![
            "*.service".into(),
            "*.socket".into(),
            "*.timer".into(),
        ])
        .await?;

    let service_files: Vec<_> = services
        .iter()
        .map(|(service_path, _)| {
            std::path::Path::new(service_path)
                .file_name()
                .unwrap()
                .to_owned()
                .into_string()
                .unwrap()
        })
        .collect();
    Ok(service_files)
}

/// Uses global dbus session to get systemd units for all users(root-level)
async fn get_enabled_system_units() -> anyhow::Result<Vec<String>> {
    let conn = zbus::Connection::system().await?;
    get_enabled_units(&conn).await
}

/// Uses current-user dbus session to get local systemd units
async fn get_enabled_user_units() -> anyhow::Result<Vec<String>> {
    let conn = zbus::Connection::session().await?;
    get_enabled_units(&conn).await
}

/// Refreshes all units cache
pub fn refresh_cache() {
    refresh_system_cache();
    refresh_user_cache();
}

/// Refreshes system-level units
pub fn refresh_system_cache() {
    MANAGER.lock().unwrap().refresh_system();
}

/// Refreshes user-level units
pub fn refresh_user_cache() {
    MANAGER.lock().unwrap().refresh_user();
}

/// Checks if units are enabled in System scope
pub fn check_system_units(units_str: &str) -> bool {
    MANAGER.lock().unwrap().system_enabled(units_str)
}

/// Checks if units are enabled in User scope
pub fn check_user_units(units_str: &str) -> bool {
    MANAGER.lock().unwrap().user_enabled(units_str)
}

/// Checks if any units enabled
pub fn check_any_units(units_str: &str) -> bool {
    MANAGER.lock().unwrap().any_enabled(units_str)
}

async fn connection_for_scope(scope: Scope) -> anyhow::Result<zbus::Connection> {
    Ok(match scope {
        Scope::System => zbus::Connection::system().await?,
        Scope::User => zbus::Connection::session().await?,
    })
}

const REPLACE_MODE: &str = "replace";

/// Call a unit-level method (`StartUnit`, `StopUnit`, `RestartUnit`) with polkit interactive auth.
async fn call_unit_method(
    manager: &zbus_systemd::systemd1::ManagerProxy<'_>,
    method: &str,
    unit: &str,
) -> zbus::Result<Option<zvariant::OwnedObjectPath>> {
    let flags = MethodFlags::AllowInteractiveAuth.into();
    manager
        .inner()
        .call_with_flags(method, flags, &(unit.to_string(), REPLACE_MODE.to_string()))
        .await
}

/// Enable units and start them
pub fn systemd_enable(units: &[&str], scope: Scope, force: bool) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let conn = connection_for_scope(scope).await?;
        let manager = zbus_systemd::systemd1::ManagerProxy::new(&conn).await?;
        let files: Vec<String> = units.iter().map(std::string::ToString::to_string).collect();
        let flags = MethodFlags::AllowInteractiveAuth.into();
        #[allow(clippy::type_complexity)]
        let _: Option<(bool, Vec<(String, String, String)>)> = manager
            .inner()
            .call_with_flags("EnableUnitFiles", flags, &(files, false, force))
            .await?;
        for unit in units {
            call_unit_method(&manager, "StartUnit", unit).await?;
        }
        Ok(())
    })
}

/// Stop units and disable them
pub fn systemd_disable(units: &[&str], scope: Scope) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let conn = connection_for_scope(scope).await?;
        let manager = zbus_systemd::systemd1::ManagerProxy::new(&conn).await?;
        for unit in units {
            // NOTE: unit may already be inactive
            let _ = call_unit_method(&manager, "StopUnit", unit).await;
        }
        let files: Vec<String> = units.iter().map(std::string::ToString::to_string).collect();
        let flags = MethodFlags::AllowInteractiveAuth.into();
        let _: Option<Vec<(String, String, String)>> =
            manager.inner().call_with_flags("DisableUnitFiles", flags, &(files, false)).await?;
        Ok(())
    })
}

/// Restart a single unit
pub fn systemd_restart(unit: &str, scope: Scope) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let conn = connection_for_scope(scope).await?;
        let manager = zbus_systemd::systemd1::ManagerProxy::new(&conn).await?;
        call_unit_method(&manager, "RestartUnit", unit).await?;
        Ok(())
    })
}

/// Stop a single unit
pub fn systemd_stop(unit: &str, scope: Scope) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let conn = connection_for_scope(scope).await?;
        let manager = zbus_systemd::systemd1::ManagerProxy::new(&conn).await?;
        call_unit_method(&manager, "StopUnit", unit).await?;
        Ok(())
    })
}

/// Check whether a unit is currently active.
pub fn systemd_is_active(unit: &str, scope: Scope) -> anyhow::Result<bool> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let conn = connection_for_scope(scope).await?;
        let manager = zbus_systemd::systemd1::ManagerProxy::new(&conn).await?;
        let path = manager.get_unit(unit.to_string()).await?;
        let unit_proxy =
            zbus_systemd::systemd1::UnitProxy::builder(&conn).path(path)?.build().await?;
        Ok(unit_proxy.active_state().await? == "active")
    })
}
