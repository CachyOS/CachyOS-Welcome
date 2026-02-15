use crate::cli::{AppToLaunch, FixAction, TweakAction};
use crate::dns::DnsAction;
use crate::tweak::{self, TweakName};
use crate::ui::UI;
use crate::{actions, dns, systemd_units, utils};

use anyhow::Result;
use colored::Colorize;
use gtk::glib;

use subprocess::Exec;

pub fn handle_fix_command(action: FixAction) -> Result<()> {
    let (tx, rx) = glib::MainContext::channel(glib::Priority::default());

    match action {
        FixAction::UpdateSystem => {
            println!("{}", "Updating system...".bold());
            actions::update_system(crate::cli::run_command);
        },
        FixAction::ReinstallPackages => {
            println!("{}", "Reinstalling all native packages...".bold());
            actions::reinstall_packages(crate::cli::run_command);
        },
        FixAction::ResetKeyrings => {
            println!("{}", "Resetting pacman keyrings...".bold());
            actions::reset_keyring(crate::cli::run_command);
        },
        FixAction::RemoveLock => {
            println!("{}", "Removing pacman database lock...".bold());
            let tx_clone = tx.clone();
            actions::remove_dblock(tx_clone);
        },
        FixAction::ClearCache => {
            println!("{}", "Clearing package cache...".bold());
            actions::clear_pkgcache(crate::cli::run_command);
        },
        FixAction::RemoveOrphans => {
            println!("{}", "Removing orphan packages...".bold());
            let tx_clone = tx.clone();
            actions::remove_orphans(crate::cli::run_command, tx_clone);
        },
        FixAction::RankMirrors => {
            println!("{}", "Ranking mirrors...".bold());
            actions::rankmirrors(crate::cli::run_command);
        },
        FixAction::InstallGaming => {
            println!("{}", "Installing CachyOS gaming packages...".bold());
            actions::install_gaming(crate::cli::run_command, tx);
        },
        FixAction::InstallSnapper => {
            if !utils::is_root_on_btrfs() {
                anyhow::bail!("Snapper requires a BTRFS root filesystem.");
            }
            println!("{}", "Installing Snapper support...".bold());
            actions::install_snapper(crate::cli::run_command, tx);
        },
        FixAction::ShowKwinDebug => {
            println!("{}", "Attempting to launch KWin debug console...".bold());
            actions::launch_kwin_debug_window();
        },
        FixAction::InstallWinboat => {
            println!("{}", "Installing Winboat...".bold());
            actions::install_winboat(crate::cli::run_command, tx);
        },
    }

    rx.attach(None, move |msg| {
        let ui_comp = crate::cli::CLI::new();
        ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
        glib::ControlFlow::Continue
    });
    Ok(())
}

pub fn handle_tweak_command(action: TweakAction) -> Result<()> {
    match action {
        TweakAction::Enable { tweak_name } => toggle_tweak_cli(tweak_name, true),
        TweakAction::Disable { tweak_name } => toggle_tweak_cli(tweak_name, false),
        TweakAction::List => list_tweaks(),
    }
}

pub fn handle_dns_command(action: DnsAction) -> Result<()> {
    let (tx, rx) = glib::MainContext::channel(glib::Priority::default());

    match action {
        DnsAction::Set { connection, server } => {
            println!("Setting DNS for '{}' to '{}'...", connection.cyan(), server.as_str().cyan());
            let server_addr = dns::G_DNS_SERVERS.get(server.as_str()).unwrap();
            actions::change_dns_server(&connection, server_addr.0, server_addr.1, tx);
        },
        DnsAction::Reset { connection } => {
            println!("Resetting DNS for '{}' to automatic...", connection.cyan());
            actions::reset_dns_server(&connection, tx);
        },
        DnsAction::ListConnections => {
            println!("{}", "Available Network Connections:".bold());
            let connections = actions::get_nm_connections();
            if connections.is_empty() {
                println!("No connections found.");
            } else {
                for conn in connections {
                    println!("- {conn}");
                }
            }
        },
        DnsAction::ListServers => {
            println!("{}", "Available DNS Servers:".bold());
            for name in dns::G_DNS_SERVERS.keys() {
                println!("- {name}");
            }
        },
    }
    rx.attach(None, move |msg| {
        let ui_comp = crate::cli::CLI::new();
        ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
        glib::ControlFlow::Continue
    });
    Ok(())
}

pub fn handle_launch_command(app: AppToLaunch) -> Result<()> {
    let (app_name, bin_name) = match app {
        AppToLaunch::PackageInstaller => ("CachyOS Package Installer", "cachyos-pi"),
        AppToLaunch::KernelManager => ("CachyOS Kernel Manager", "cachyos-kernel-manager"),
    };

    println!("Launching {}...", app_name.bold());

    match which::which(bin_name) {
        Ok(path) => {
            Exec::cmd(path).detached().join()?;
            println!("{app_name} launched successfully.");
        },
        Err(_) => {
            anyhow::bail!("'{bin_name}' executable not found in your PATH.");
        },
    }
    Ok(())
}

fn toggle_tweak_cli(tweak: TweakName, enable: bool) -> Result<()> {
    let (action_type, action_data, alpm_package_name) = tweak::get_details(tweak);

    let verb = if enable { "Enabling" } else { "Disabling" };
    println!("{verb} tweak '{tweak:?}'...");

    // If enabling, ensure package is installed first
    if enable && !alpm_package_name.is_empty() && !utils::is_alpm_pkg_installed(alpm_package_name) {
        println!(
            "Required package '{}' is not installed. Installing...",
            alpm_package_name.yellow()
        );
        let status =
            crate::cli::run_command(&format!("pacman -S --noconfirm {alpm_package_name}"), true);
        if !status || !utils::is_alpm_pkg_installed(alpm_package_name) {
            anyhow::bail!(
                "Failed to install required package '{alpm_package_name}'. Cannot enable tweak."
            );
        }
    }

    let (cmd, run_as_root) = utils::get_tweak_toggle_cmd(action_type, action_data, !enable);

    println!("> {}", cmd.cyan());
    let exit_status = utils::run_cmd(cmd, run_as_root).unwrap();
    if !exit_status.success() {
        anyhow::bail!(
            "Failed to {} tweak '{:?}'. Command exited with error.",
            verb.to_lowercase(),
            tweak
        );
    }

    let status = if enable { "enabled".green() } else { "disabled".red() };
    println!("Tweak '{tweak:?}' successfully {status}.");
    Ok(())
}

fn list_tweaks() -> Result<()> {
    println!("{}", "Available Tweaks Status:".bold());

    // Get all enabled units
    systemd_units::refresh_cache();

    for tweak in &[
        TweakName::Psd,
        TweakName::Oomd,
        TweakName::Bpftune,
        TweakName::Bluetooth,
        TweakName::Ananicy,
        TweakName::CachyUpdate,
    ] {
        let (_, service_names, _) = tweak::get_details(*tweak);
        let is_enabled = systemd_units::check_any_units(service_names);

        let status = if is_enabled { "[enabled]".green() } else { "[disabled]".red() };

        println!("- {:<12} {}", format!("{:?}", tweak), status);
    }

    Ok(())
}
