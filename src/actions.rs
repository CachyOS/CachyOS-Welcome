use crate::ui::{Action, DialogMessage, MessageType, RunCmdCallback};
use crate::{dns, fl, kwin_dbus, systemd_units, utils, PacmanWrapper};

use std::env;
use std::path::Path;

use gtk::glib::Sender;
use subprocess::{Exec, Redirection};
use tracing::error;

pub fn get_nm_connections() -> Vec<String> {
    let connections = Exec::cmd("/sbin/nmcli")
        .args(&["-t", "-f", "NAME", "connection", "show"])
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    // get list of connections separated by newline
    connections.split('\n').filter(|x| !x.is_empty()).map(String::from).collect::<Vec<_>>()
}

pub fn get_active_connection_name() -> Option<String> {
    let active_conns = Exec::cmd("/sbin/nmcli")
        .args(&["-g", "NAME", "connection", "show", "--active"])
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    active_conns.lines().next().map(String::from)
}

/// DNS info returned from NetworkManager: (ipv4_addrs, ipv6_addrs, optional DoT hostname).
/// The hostname is extracted from the NM `address#hostname` notation.
pub struct DnsInfo {
    pub ipv4: String,
    pub ipv6: String,
    pub dot_hostname: Option<String>,
}

pub fn get_dns_for_connection(conn_name: &str) -> Option<DnsInfo> {
    let ips = Exec::cmd("/sbin/nmcli")
        .args(&["-g", "ipv4.dns,ipv6.dns", "con", "show", conn_name])
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    let mut lines = ips.lines();
    let raw_ipv4 = lines.next().unwrap_or("").to_owned();
    let raw_ipv6 = lines.next().unwrap_or("").replace("\\:", ":");

    if raw_ipv4.is_empty() && raw_ipv6.is_empty() {
        return None;
    }

    // Extract DoT hostname from "addr#hostname" notation.
    // All addresses in a connection share the same hostname, so take the first found.
    let mut dot_hostname: Option<String> = None;
    let strip_hostname = |s: &str, hostname: &mut Option<String>| -> String {
        s.split(',')
            .map(|addr| {
                if let Some(pos) = addr.find('#') {
                    if hostname.is_none() {
                        *hostname = Some(addr[pos + 1..].to_string());
                    }
                    addr[..pos].to_string()
                } else {
                    addr.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    };

    let ipv4 = strip_hostname(&raw_ipv4, &mut dot_hostname);
    let ipv6 = strip_hostname(&raw_ipv6, &mut dot_hostname);

    Some(DnsInfo { ipv4, ipv6, dot_hostname })
}

/// Returns true if DNS-over-TLS is enabled (strict mode) for the given connection.
pub fn get_dot_for_connection(conn_name: &str) -> bool {
    let output = Exec::cmd("/sbin/nmcli")
        .args(&["-g", "connection.dns-over-tls", "con", "show", conn_name])
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();
    // value 2 = strict DoT
    output.trim() == "2"
}

fn get_user_groups() -> Vec<String> {
    let groups =
        Exec::cmd("/sbin/groups").stdout(Redirection::Pipe).capture().unwrap().stdout_str();
    groups.split('\n').filter(|x| !x.is_empty()).map(String::from).collect::<Vec<_>>()
}

pub fn launch_kwin_debug_window() {
    if let Err(kwin_err) = kwin_dbus::launch_kwin_debug_window() {
        error!("Failed to launch kwin debug window: {kwin_err}");
    }
}

pub fn change_dns_server(
    conn_name: &str,
    server_addr_ipv4: &str,
    server_addr_ipv6: &str,
    enable_dot: bool,
    dot_hostname: &str,
    dialog_tx: Sender<DialogMessage>,
) {
    // When DoT is enabled and a hostname is provided, append #hostname to each address
    // per NetworkManager's "address#servername" notation for SNI.
    let ipv4_with_sni = if enable_dot && !dot_hostname.is_empty() {
        dns::append_dot_hostname(server_addr_ipv4, dot_hostname)
    } else {
        server_addr_ipv4.to_string()
    };
    let ipv6_with_sni = if enable_dot && !dot_hostname.is_empty() {
        dns::append_dot_hostname(server_addr_ipv6, dot_hostname)
    } else {
        server_addr_ipv6.to_string()
    };

    // dns-over-tls: -1 = default, 0 = no, 1 = opportunistic, 2 = yes (strict)
    let dot_value = if enable_dot { 2 } else { 0 };
    let status_code = utils::run_cmd(
        format!(
            "nmcli con mod '{conn_name}' ipv4.dns '{ipv4_with_sni}' && \
             nmcli con mod '{conn_name}' ipv4.dns-priority -1 && \
             nmcli con mod '{conn_name}' ipv6.dns '{ipv6_with_sni}' && \
             nmcli con mod '{conn_name}' ipv6.dns-priority -1 && \
             nmcli con mod '{conn_name}' connection.dns-over-tls {dot_value} && \
             systemctl restart NetworkManager"
        ),
        true,
    )
    .unwrap();
    if status_code.success() {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-changed"),
                msg_type: MessageType::Info,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    } else {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-failed"),
                msg_type: MessageType::Error,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    }
}

pub fn reset_dns_server(conn_name: &str, dialog_tx: Sender<DialogMessage>) {
    // Stop blocky if it was running (DoH mode)
    stop_blocky();

    let conn = conn_name.replace('\'', "'\\'");
    let status_code = utils::run_cmd(
        format!(
            "nmcli con mod '{conn}' ipv4.dns '' && nmcli con mod '{conn}' ipv6.dns '' \\
             && nmcli con mod '{conn}' ipv4.dns-priority 0 \\
             && nmcli con mod '{conn}' ipv6.dns-priority 0 \\
             && nmcli con mod '{conn}' ipv4.ignore-auto-dns no \\
             && nmcli con mod '{conn}' ipv6.ignore-auto-dns no \\
             && nmcli con mod '{conn}' connection.dns-over-tls -1 \\
             && systemctl restart NetworkManager"
        ),
        true,
    )
    .unwrap();
    if status_code.success() {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-reset"),
                msg_type: MessageType::Info,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    } else {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-reset-failed"),
                msg_type: MessageType::Error,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    }
}

/// Set DNS to use DoH via blocky local proxy.
/// Installs blocky if needed, writes its config, starts the service, and points NM to 127.0.0.1.
pub fn change_dns_server_doh(
    callback: RunCmdCallback,
    conn_name: &str,
    doh_url: &str,
    bootstrap_ipv4: &str,
    bootstrap_ipv6: &str,
    dot_hostname: Option<&str>,
    dialog_tx: Sender<DialogMessage>,
) {
    // 1. Install blocky if not present
    if !utils::is_alpm_pkg_installed("blocky") {
        const ALPM_PACKAGE_NAMES: [&str; 1] = ["blocky"];
        install_needed_packages(
            callback,
            &ALPM_PACKAGE_NAMES,
            fl!("doh-blocky-install-failed"),
            Action::SetDnsServer,
            dialog_tx.clone(),
        );
        if !utils::is_alpm_pkg_installed("blocky") {
            return;
        }
    }

    // 2. Generate and write blocky config
    let config = dns::generate_blocky_config(doh_url, bootstrap_ipv4, bootstrap_ipv6, dot_hostname);

    let escaped_config = config.replace('\'', "'\\''");
    let write_status = utils::run_cmd(
        format!(
            "mkdir -p /etc/blocky && printf '%s' '{}' > {}",
            escaped_config,
            dns::BLOCKY_CONFIG_PATH,
        ),
        true,
    )
    .unwrap();
    if !write_status.success() {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-failed"),
                msg_type: MessageType::Error,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
        return;
    }

    // 3. Configure NM, restart NM, then (re)start blocky once network is back
    // Use ignore-auto-dns to ensure all DNS goes through blocky — DHCP DNS
    // would bypass the encrypted proxy. LAN names still work via mDNS/LLMNR.
    let conn = conn_name.replace('\'', "'\\'");
    let status_code = utils::run_cmd(
        format!(
            "systemctl enable {blocky} && \\
             nmcli con mod '{conn}' ipv4.dns '127.0.0.1' && \\
             nmcli con mod '{conn}' ipv4.ignore-auto-dns yes && \\
             nmcli con mod '{conn}' ipv6.dns '::1' && \\
             nmcli con mod '{conn}' ipv6.ignore-auto-dns yes && \\
             nmcli con mod '{conn}' connection.dns-over-tls 0 && \
             systemctl restart NetworkManager && \
             sleep 1 && \
             systemctl restart {blocky}",
            blocky = dns::BLOCKY_SERVICE,
        ),
        true,
    )
    .unwrap();

    if status_code.success() {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-changed"),
                msg_type: MessageType::Info,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    } else {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-failed"),
                msg_type: MessageType::Error,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    }
}

/// Stop blocky if it's running (used during reset or when switching away from DoH).
pub fn stop_blocky() {
    let (cmd, run_as_root) =
        utils::get_tweak_toggle_cmd("service", dns::BLOCKY_SERVICE, true);
    let _ = utils::run_cmd(cmd, run_as_root);
}

/// Returns true if blocky is currently active.
pub fn is_blocky_active() -> bool {
    utils::run_cmd(
        format!("systemctl is-active --quiet {}", dns::BLOCKY_SERVICE),
        false,
    )
    .is_ok_and(|s| s.success())
}

pub fn remove_dblock(dialog_tx: Sender<DialogMessage>) {
    if Path::new("/var/lib/pacman/db.lck").exists() {
        let _ = utils::run_cmd("rm /var/lib/pacman/db.lck".into(), true).unwrap();
        if !Path::new("/var/lib/pacman/db.lck").exists() {
            dialog_tx
                .send(DialogMessage {
                    msg: fl!("removed-db-lock"),
                    msg_type: MessageType::Info,
                    action: Action::RemoveLock,
                })
                .expect("Couldn't send data to channel");
        }
    } else {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("lock-doesnt-exist"),
                msg_type: MessageType::Info,
                action: Action::RemoveLock,
            })
            .expect("Couldn't send data to channel");
    }
}

pub fn update_system(callback: RunCmdCallback) {
    let (cmd, escalate) = match utils::get_pacman_wrapper() {
        PacmanWrapper::Aura => ("aura -Syu && aura -Akaxu", false),
        _ => ("pacman -Syu", true),
    };
    let _ = utils::run_cmd_terminal(callback, String::from(cmd), escalate);
}

pub fn clear_pkgcache(callback: RunCmdCallback) {
    let (cmd, escalate) = match utils::get_pacman_wrapper() {
        PacmanWrapper::Pak => ("pak -Sc", false),
        PacmanWrapper::Yay => ("yay -Sc", false),
        PacmanWrapper::Paru => ("paru -Sc", false),
        _ => ("pacman -Sc", true),
    };
    let _ = utils::run_cmd_terminal(callback, String::from(cmd), escalate);
}

pub fn reinstall_packages(callback: RunCmdCallback) {
    let _ = utils::run_cmd_terminal(callback, String::from("pacman -S $(pacman -Qnq)"), true);
}

pub fn remove_orphans(callback: RunCmdCallback, dialog_tx: Sender<DialogMessage>) {
    // check if you have orphans packages.
    let mut orphan_pkgs = Exec::cmd("/sbin/pacman")
        .arg("-Qtdq")
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    // get list of packages separated by space,
    // and check if it's empty or not.
    orphan_pkgs = orphan_pkgs.replace('\n', " ");
    if orphan_pkgs.is_empty() {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("orphans-not-found"),
                msg_type: MessageType::Info,
                action: Action::RemoveOrphans,
            })
            .expect("Couldn't send data to channel");
        return;
    }
    let _ = utils::run_cmd_terminal(callback, format!("pacman -Rns {orphan_pkgs}"), true);
}

pub fn reset_keyring(callback: RunCmdCallback) {
    let key_reset = r"
rm -rf /etc/pacman.d/gnupg/ && \
pacman-key --init && pacman-key --populate && \
pacman-key --recv-keys F3B607488DB35A47 --keyserver keyserver.ubuntu.com && \
pacman-key --lsign-key F3B607488DB35A47
";

    let _ = utils::run_cmd_terminal(callback, key_reset.into(), true);
}

pub fn install_needed_packages(
    callback: RunCmdCallback,
    package_names: &[&str],
    dialog_msg: String,
    dialog_action: Action,
    dialog_tx: Sender<DialogMessage>,
) {
    let mut packages_to_install: Vec<&str> = Vec::new();
    for alpm_package_name in package_names {
        if !utils::is_alpm_pkg_installed(alpm_package_name) {
            packages_to_install.push(alpm_package_name);
        }
    }
    // skip if installed already
    if packages_to_install.is_empty() {
        dialog_tx
            .send(DialogMessage {
                msg: dialog_msg,
                msg_type: MessageType::Info,
                action: dialog_action,
            })
            .expect("Couldn't send data to channel");
        return;
    }

    // install overwise
    let packages = packages_to_install.join(" ");
    let _ = utils::run_cmd_terminal(callback, format!("pacman -S {packages}"), true);
}

pub fn rankmirrors(callback: RunCmdCallback) {
    let _ = utils::run_cmd_terminal(callback, String::from("cachyos-rate-mirrors"), true);
}

pub fn install_gaming(callback: RunCmdCallback, dialog_tx: Sender<DialogMessage>) {
    const ALPM_PACKAGE_NAMES: [&str; 2] = ["cachyos-gaming-meta", "cachyos-gaming-applications"];
    install_needed_packages(
        callback,
        &ALPM_PACKAGE_NAMES,
        fl!("gaming-package-installed"),
        Action::InstallGaming,
        dialog_tx,
    );
}

pub fn install_winboat(callback: RunCmdCallback, dialog_tx: Sender<DialogMessage>) {
    const ALPM_PACKAGE_NAMES: [&str; 3] = ["winboat", "docker", "docker-compose"];
    install_needed_packages(
        callback,
        &ALPM_PACKAGE_NAMES,
        fl!("winboat-package-installed"),
        Action::InstallWinboat,
        dialog_tx.clone(),
    );

    // Enable docker.socket after installation
    const DOCKER_TARGET: &str = "docker.socket";
    let docker_enabled = systemd_units::check_system_units(DOCKER_TARGET);
    if utils::is_alpm_pkg_installed("docker") && !docker_enabled {
        let (cmd, run_as_root) =
            utils::get_tweak_toggle_cmd("service", DOCKER_TARGET, docker_enabled);
        let status_code = utils::run_cmd(cmd, run_as_root).unwrap();
        if !status_code.success() {
            dialog_tx
                .send(DialogMessage {
                    msg: fl!("winboat-install-failed"),
                    msg_type: MessageType::Error,
                    action: Action::InstallWinboat,
                })
                .expect("Couldn't send data to channel");
        }

        // refresh units cache
        systemd_units::refresh_system_cache();
    }

    // Add the current user to the docker group
    let group_added = get_user_groups().iter().any(|x| x == "docker");
    if utils::is_alpm_pkg_installed("docker") && !group_added {
        if let Ok(current_user) = env::var("USER") {
            let status_code =
                utils::run_cmd(format!("/sbin/usermod -aG docker {current_user}"), true).unwrap();
            if !status_code.success() {
                dialog_tx
                    .send(DialogMessage {
                        msg: fl!("winboat-install-failed"),
                        msg_type: MessageType::Error,
                        action: Action::InstallWinboat,
                    })
                    .expect("Couldn't send data to channel");
            }
        }
    }
}
