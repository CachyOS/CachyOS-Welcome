#[derive(Clone, Debug)]
pub struct SystemdUnits {
    pub enabled_units: Vec<String>,
}

impl SystemdUnits {
    pub fn new() -> Self {
        Self { enabled_units: Vec::new() }
    }
}

impl Default for SystemdUnits {
    fn default() -> Self {
        Self::new()
    }
}

async fn get_enabled_units(conn: &zbus::Connection) -> anyhow::Result<Vec<String>> {
    let manager = zbus_systemd::systemd1::ManagerProxy::new(conn).await?;
    let services = manager
        .list_unit_files_by_patterns(vec!["enabled".into()], vec!["*.service".into()])
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

pub async fn get_enabled_global_units() -> anyhow::Result<Vec<String>> {
    let conn = zbus::Connection::system().await?;
    get_enabled_units(&conn).await
}

pub async fn get_enabled_user_units() -> anyhow::Result<Vec<String>> {
    let conn = zbus::Connection::session().await?;
    get_enabled_units(&conn).await
}
