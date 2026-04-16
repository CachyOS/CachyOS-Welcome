use crate::cli::{AppToLaunch, FixAction, TweakAction};
use crate::dns::DnsAction;
use crate::systemd_units::Scope;
use crate::tweak::{self, TweakName};
use crate::ui::UI;
use crate::{actions, dns, systemd_units, utils};

use anyhow::Result;
use colored::Colorize;

pub fn handle_fix_command(action: FixAction) -> Result<()> {
    let (tx, rx) = async_channel::unbounded();

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
        FixAction::ShowKwinDebug => {
            println!("{}", "Attempting to launch KWin debug console...".bold());
            actions::launch_kwin_debug_window();
        },
        FixAction::InstallWinboat => {
            println!("{}", "Installing Winboat...".bold());
            actions::install_winboat(crate::cli::run_command, tx);
        },
        FixAction::InstallVramManagement => {
            println!("{}", "Installing VRAM management packages...".bold());
            actions::install_vram_management(crate::cli::run_command, tx);
        },
    }

    while let Ok(msg) = rx.try_recv() {
        let ui_comp = crate::cli::ConsoleUi::new();
        ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
    }
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
    let (tx, rx) = async_channel::unbounded();

    match action {
        DnsAction::Set { connection, server, dot, doh } => {
            let server_name = server.as_str();
            let server_addr = dns::G_DNS_SERVERS.get(server_name).unwrap();

            if doh {
                // DoH mode via blocky
                let doh_url = dns::get_doh_url(server_name);
                if let Some(url) = doh_url {
                    println!(
                        "Setting DNS for '{}' to '{}' (DoH enabled via blocky)...",
                        connection.cyan(),
                        server_name.cyan(),
                    );
                    actions::change_dns_server_doh(
                        crate::cli::run_command,
                        &connection,
                        url,
                        server_addr.0,
                        server_addr.1,
                        server_addr.2,
                        tx,
                    );
                } else {
                    println!(
                        "{}: DNS over HTTPS is not supported by '{}'.",
                        "Warning".yellow(),
                        server_name
                    );
                    println!("Setting DNS without DoH...");
                    let dot_hostname = server_addr.2.unwrap_or("");
                    actions::change_dns_server(
                        &connection,
                        server_addr.0,
                        server_addr.1,
                        false,
                        dot_hostname,
                        tx,
                    );
                }
            } else {
                // Stop blocky if switching away from DoH
                actions::stop_blocky();
                let dot_supported = server_addr.2.is_some();

                if dot && !dot_supported {
                    println!(
                        "{}: DNS over TLS is not supported by '{}'.",
                        "Warning".yellow(),
                        server_name
                    );
                    println!("Setting DNS without DoT...");
                }

                let enable_dot = dot && dot_supported;
                let dot_label = if enable_dot { " (DoT enabled)" } else { "" };
                let dot_hostname = server_addr.2.unwrap_or("");
                println!(
                    "Setting DNS for '{}' to '{}'{}...",
                    connection.cyan(),
                    server_name.cyan(),
                    dot_label
                );
                actions::change_dns_server(
                    &connection,
                    server_addr.0,
                    server_addr.1,
                    enable_dot,
                    dot_hostname,
                    tx,
                );
            }
        },
        DnsAction::SetCustom { connection, ipv4, ipv6, dot, dot_hostname, doh, doh_url } => {
            if ipv4.is_empty() && ipv6.is_empty() {
                eprintln!("{}: At least one of --ipv4 or --ipv6 must be provided.", "Error".red());
                std::process::exit(1);
            }
            if !dot_hostname.is_empty() && !dns::is_valid_dot_hostname(&dot_hostname) {
                eprintln!("{}: Invalid DoT hostname '{}'.", "Error".red(), dot_hostname);
                std::process::exit(1);
            }

            if doh {
                if doh_url.is_empty() || !doh_url.starts_with("https://") {
                    eprintln!("{}: --doh-url must be a valid https:// URL.", "Error".red());
                    std::process::exit(1);
                }
                println!(
                    "Setting custom DoH DNS for '{}': URL='{}' (bootstrap: IPv4='{}' \
                     IPv6='{}'{})...",
                    connection.cyan(),
                    doh_url.cyan(),
                    if ipv4.is_empty() { "(none)" } else { &ipv4 },
                    if ipv6.is_empty() { "(none)" } else { &ipv6 },
                    if dot_hostname.is_empty() {
                        String::new()
                    } else {
                        format!(" DoT bootstrap={dot_hostname}")
                    },
                );
                let dot_host =
                    if dot_hostname.is_empty() { None } else { Some(dot_hostname.as_str()) };
                actions::change_dns_server_doh(
                    crate::cli::run_command,
                    &connection,
                    &doh_url,
                    &ipv4,
                    &ipv6,
                    dot_host,
                    tx,
                );
            } else {
                // Stop blocky if switching away from DoH
                actions::stop_blocky();
                let dot_label = if dot { " (DoT enabled)" } else { "" };
                println!(
                    "Setting custom DNS for '{}': IPv4='{}' IPv6='{}'{}{}",
                    connection.cyan(),
                    if ipv4.is_empty() { "(none)" } else { &ipv4 },
                    if ipv6.is_empty() { "(none)" } else { &ipv6 },
                    if dot_hostname.is_empty() {
                        String::new()
                    } else {
                        format!(" hostname={dot_hostname}")
                    },
                    dot_label,
                );
                actions::change_dns_server(&connection, &ipv4, &ipv6, dot, &dot_hostname, tx);
            }
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
            for (name, (_, _, dot_hostname)) in dns::G_DNS_SERVERS.entries() {
                let dot_info = match dot_hostname {
                    Some(host) => format!(" [DoT: {host}]"),
                    None => String::new(),
                };
                let doh_info = match dns::get_doh_url(name) {
                    Some(url) => format!(" [DoH: {url}]"),
                    None => String::new(),
                };
                let region_info = match dns::G_DNS_SERVER_INFO.get(name) {
                    Some(info) => format!(" ({} - {})", info.region, info.homepage),
                    None => String::new(),
                };
                println!("- {name}{dot_info}{doh_info}{region_info}");
            }
        },
        DnsAction::TestLatency => {
            println!("{}", "Testing latency to all DNS servers...".bold());
            let results = dns::measure_all_latencies();
            for (name, latency) in &results {
                match latency {
                    Some(ms) => println!("  {ms:>4} ms  {name}"),
                    None => println!("     --  {name} (timeout)"),
                }
            }
        },
    }
    while let Ok(msg) = rx.try_recv() {
        let ui_comp = crate::cli::ConsoleUi::new();
        ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
    }
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
            utils::spawn_detached(path.to_str().unwrap())?;
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

    let scope = if action_type == "user_service" { Scope::User } else { Scope::System };
    let units: Vec<&str> = action_data.split_whitespace().collect();

    println!("> {} {}", verb, action_data.cyan());
    let result = if enable {
        systemd_units::systemd_enable(&units, scope, true)
    } else {
        systemd_units::systemd_disable(&units, scope)
    };
    if let Err(e) = result {
        anyhow::bail!("Failed to {} tweak '{:?}': {e}", verb.to_lowercase(), tweak);
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
