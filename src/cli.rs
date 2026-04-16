use crate::ui::{MessageType, UI};
use crate::{dns, tweak, utils};

use clap::{Args, Parser, Subcommand};

pub struct ConsoleUi;

impl ConsoleUi {
    pub fn new() -> Self {
        Self {}
    }
}

impl UI for ConsoleUi {
    fn show_message(&self, message_type: MessageType, message: &str, title: String) {
        println!("{}", format_message(message_type, message, &title));
    }
}

fn format_message(message_type: MessageType, message: &str, title: &str) -> String {
    let type_str = match message_type {
        MessageType::Info => "INFO",
        MessageType::Warning => "WARNING",
        MessageType::Error => "ERROR",
    };
    format!("[{type_str}] {title}: {message}")
}

pub fn run_command(command: &str, escalate: bool) -> bool {
    let status = utils::run_cmd(command.into(), escalate).expect("failed to run cmd");
    status.success()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(subcommand_negates_reqs = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// TODO(vnepogodin): move all that to unified location to be used within GUI too
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Perform common system maintenance and repair tasks
    Fix(FixArgs),
    /// Enable or disable tweaks
    Tweak(TweakArgs),
    /// Configure DNS settings for network connections
    Dns(DnsArgs),
    /// Launch associated `CachyOS` applications
    Launch(LaunchArgs),
}

#[derive(Args, Debug)]
pub struct FixArgs {
    #[clap(subcommand)]
    pub action: FixAction,
}

#[derive(Subcommand, Debug)]
pub enum FixAction {
    /// Update the system via package manager
    UpdateSystem,
    /// Reinstall all native packages
    ReinstallPackages,
    /// Reset and repopulate pacman keyrings
    ResetKeyrings,
    /// Remove the pacman database lock file
    RemoveLock,
    /// Clear the pacman package cache
    ClearCache,
    /// Remove orphan packages from the system
    RemoveOrphans,
    /// Rank mirrors to find up2date&fastest ones
    RankMirrors,
    /// Install `CachyOS` gaming meta-packages
    InstallGaming,
    /// Show the `KWin` Wayland debug console (if running)
    ShowKwinDebug,
    /// Install Winboat for Windows applications
    InstallWinboat,
    /// Install VRAM management packages
    InstallVramManagement,
}

#[derive(Args, Debug)]
pub struct TweakArgs {
    #[clap(subcommand)]
    pub action: TweakAction,
}

#[derive(Subcommand, Debug)]
pub enum TweakAction {
    /// Enable a specific tweak (starts and enables a systemd service/timer)
    Enable {
        /// The tweak to enable.
        #[clap(value_enum)]
        tweak_name: tweak::TweakName,
    },
    /// Disable a specific tweak (stops and disables a systemd service/timer)
    Disable {
        /// The tweak to disable.
        #[clap(value_enum)]
        tweak_name: tweak::TweakName,
    },
    /// List available tweaks and their current status
    List,
}

#[derive(Args, Debug)]
pub struct DnsArgs {
    #[clap(subcommand)]
    pub action: dns::DnsAction,
}

#[derive(Args, Debug)]
pub struct LaunchArgs {
    #[clap(subcommand)]
    pub app: AppToLaunch,
}

#[derive(Subcommand, Debug)]
pub enum AppToLaunch {
    /// Launch the `CachyOS` Package Installer
    PackageInstaller,
    /// Launch the `CachyOS` Kernel Manager
    KernelManager,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ui::MessageType;

    #[test]
    fn show_message_formats_correctly() {
        assert_eq!(
            format_message(MessageType::Info, "test message", "Title"),
            "[INFO] Title: test message"
        );
        assert_eq!(
            format_message(MessageType::Warning, "test message", "Title"),
            "[WARNING] Title: test message"
        );
        assert_eq!(
            format_message(MessageType::Error, "test message", "Title"),
            "[ERROR] Title: test message"
        );
    }
}
