use anyhow::{Context, Result};
use tokio::runtime::Runtime;
use zbus::Connection;

#[zbus::proxy(interface = "org.kde.KWin", default_service = "org.kde.KWin", default_path = "/KWin")]
pub trait KWin {
    /// showDebugConsole method
    #[zbus(name = "showDebugConsole")]
    fn show_debug_console(&self) -> zbus::Result<()>;
}

pub fn launch_kwin_debug_window() -> Result<()> {
    let rt = Runtime::new().context("Failed to initialize tokio runtime")?;
    rt.block_on(async move {
        let connection = Connection::session().await?;
        let kwin_client = KWinProxy::new(&connection).await?;
        kwin_client.show_debug_console().await?;

        anyhow::Ok(())
    })
}
