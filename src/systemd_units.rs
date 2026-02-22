use std::sync::{LazyLock, Mutex};

use tokio::runtime::Runtime;
use tracing::error;

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
