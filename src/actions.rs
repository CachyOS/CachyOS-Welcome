use crate::networkmanager::{self, DnsMods, NmDnsOverTls};
use crate::systemd_units::Scope;
use crate::ui::{Action, DialogMessage, MessageType, RunCmdCallback};
use crate::{PacmanWrapper, dns, fl, kwin_dbus, systemd_units, utils};

use std::path::Path;
use std::time::Duration;
use std::{env, io, thread};

use async_channel::Sender;
use tracing::error;

fn split_dns_addrs(addrs: &str) -> Vec<String> {
    addrs.split(',').filter(|s| !s.is_empty()).map(String::from).collect()
}

/// Which DNS operation produced a result, for picking the dialog text.
#[derive(Clone, Copy)]
enum DnsOp {
    Change,
    Reset,
}

/// Map a [`networkmanager::modify_connection_dns`] outcome to a dialog.
fn send_dns_result(
    dialog_tx: Sender<DialogMessage>,
    conn_name: &str,
    result: anyhow::Result<networkmanager::ApplyStatus>,
    operation: DnsOp,
) {
    let (msg, msg_type) = match result {
        Ok(networkmanager::ApplyStatus::Applied) => {
            let success = match operation {
                DnsOp::Reset => fl!("dns-server-reset"),
                DnsOp::Change => fl!("dns-server-changed"),
            };
            (success, MessageType::Info)
        },
        Ok(networkmanager::ApplyStatus::PendingReconnect) => {
            (fl!("dns-server-pending"), MessageType::Warning)
        },
        Err(ref dns_err) => {
            error!("DNS operation failed for connection '{conn_name}': {dns_err}");
            let fail = match operation {
                DnsOp::Reset => fl!("dns-server-reset-failed"),
                DnsOp::Change => fl!("dns-server-failed"),
            };
            (fail, MessageType::Error)
        },
    };
    dialog_tx
        .send_blocking(DialogMessage { msg, msg_type, action: Action::SetDnsServer })
        .expect("Couldn't send data to channel");
}

pub fn get_nm_connections() -> Vec<String> {
    let connections = utils::cmd_output("/sbin/nmcli", &["-t", "-f", "NAME", "connection", "show"]);

    // get list of connections separated by newline
    connections.split('\n').filter(|x| !x.is_empty()).map(String::from).collect::<Vec<_>>()
}

pub fn get_active_connection_name() -> Option<String> {
    let active_conns =
        utils::cmd_output("/sbin/nmcli", &["-g", "NAME", "connection", "show", "--active"]);

    active_conns.lines().next().map(String::from)
}

/// DNS info returned from `NetworkManager`: (`ipv4_addrs`, `ipv6_addrs`, optional `DoT` hostname).
/// The hostname is extracted from the NM `address#hostname` notation.
pub struct DnsInfo {
    pub ipv4: String,
    pub ipv6: String,
    pub dot_hostname: Option<String>,
}

pub fn get_dns_for_connection(conn_name: &str) -> Option<DnsInfo> {
    let ips =
        utils::cmd_output("/sbin/nmcli", &["-g", "ipv4.dns,ipv6.dns", "con", "show", conn_name]);

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
    let output = utils::cmd_output("/sbin/nmcli", &[
        "-g",
        "connection.dns-over-tls",
        "con",
        "show",
        conn_name,
    ]);
    // value 2 = strict DoT
    output.trim() == "2"
}

fn get_user_groups() -> Vec<String> {
    let groups = utils::cmd_output("/sbin/groups", &[]);
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
    // When DoT is enabled with a hostname, append #hostname to each address per
    // NetworkManager's "address#servername" SNI notation.
    let with_sni = |addr: &str| {
        if enable_dot && !dot_hostname.is_empty() {
            dns::append_dot_hostname(addr, dot_hostname)
        } else {
            addr.to_string()
        }
    };

    let dot = if enable_dot { NmDnsOverTls::Yes } else { NmDnsOverTls::No };
    let mods = DnsMods {
        ipv4_dns: Some(split_dns_addrs(&with_sni(server_addr_ipv4))),
        ipv6_dns: Some(split_dns_addrs(&with_sni(server_addr_ipv6))),
        ipv4_dns_priority: Some(-1),
        ipv6_dns_priority: Some(-1),
        dns_over_tls: Some(dot),
        ..Default::default()
    };
    send_dns_result(
        dialog_tx,
        conn_name,
        networkmanager::modify_connection_dns(conn_name, &mods),
        DnsOp::Change,
    );
}

pub fn reset_dns_server(conn_name: &str, dialog_tx: Sender<DialogMessage>) {
    // Stop blocky if it was running (DoH/DoQ mode)
    stop_blocky();

    let mods = DnsMods {
        ipv4_dns: Some(Vec::new()),
        ipv6_dns: Some(Vec::new()),
        ipv4_dns_priority: Some(0),
        ipv6_dns_priority: Some(0),
        ipv4_ignore_auto_dns: Some(false),
        ipv6_ignore_auto_dns: Some(false),
        dns_over_tls: Some(NmDnsOverTls::Default),
    };
    send_dns_result(
        dialog_tx,
        conn_name,
        networkmanager::modify_connection_dns(conn_name, &mods),
        DnsOp::Reset,
    );
}

/// Set DNS to use an encrypted upstream via blocky local proxy.
/// Installs blocky if needed, writes its config, starts the service, and points NM to 127.0.0.1.
#[allow(clippy::too_many_arguments)]
pub fn change_dns_server_blocky(
    callback: RunCmdCallback,
    conn_name: &str,
    mode: dns::BlockyMode,
    upstream: &str,
    bootstrap_ipv4: &str,
    bootstrap_ipv6: &str,
    dot_hostname: Option<&str>,
    dialog_tx: Sender<DialogMessage>,
) {
    let install_failed_msg = match mode {
        dns::BlockyMode::Doh => fl!("blocky-install-failed", mode = "DoH"),
        dns::BlockyMode::Doq => fl!("blocky-install-failed", mode = "DoQ"),
    };

    // 1. Install blocky if not present
    if !utils::is_alpm_pkg_installed("blocky") {
        const ALPM_PACKAGE_NAMES: [&str; 1] = ["blocky"];
        install_needed_packages(
            callback,
            &ALPM_PACKAGE_NAMES,
            install_failed_msg,
            Action::SetDnsServer,
            dialog_tx.clone(),
        );
        if !utils::is_alpm_pkg_installed("blocky") {
            return;
        }
    }

    // 2. Generate and write blocky config
    let config =
        dns::generate_blocky_config(upstream, bootstrap_ipv4, bootstrap_ipv6, dot_hostname);

    let write_result = (|| -> anyhow::Result<()> {
        let mut tmp = tempfile::NamedTempFile::new()?;
        io::Write::write_all(&mut tmp, config.as_bytes())?;
        let status = utils::pkexec_cmd(&[
            "install",
            "-Dm644",
            tmp.path().to_str().unwrap(),
            dns::BLOCKY_CONFIG_PATH,
        ])?;
        anyhow::ensure!(status.success(), "failed to write blocky config");
        Ok(())
    })();
    if let Err(write_err) = write_result {
        error!("Failed to write blocky config: {write_err}");
        dialog_tx
            .send_blocking(DialogMessage {
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
    let result = (|| -> anyhow::Result<networkmanager::ApplyStatus> {
        systemd_units::systemd_enable(&[dns::BLOCKY_SERVICE], Scope::System, false)?;
        let mods = DnsMods {
            ipv4_dns: Some(vec![String::from("127.0.0.1")]),
            ipv6_dns: Some(vec![String::from("::1")]),
            ipv4_ignore_auto_dns: Some(true),
            ipv6_ignore_auto_dns: Some(true),
            dns_over_tls: Some(NmDnsOverTls::No),
            ..Default::default()
        };
        let status = networkmanager::modify_connection_dns(conn_name, &mods)?;
        thread::sleep(Duration::from_secs(1));
        systemd_units::systemd_restart(dns::BLOCKY_SERVICE, Scope::System)?;
        Ok(status)
    })();

    send_dns_result(dialog_tx, conn_name, result, DnsOp::Change);
}

/// Stop blocky if it's running (used during reset or when switching away from encrypted DNS).
pub fn stop_blocky() {
    let _ = systemd_units::systemd_stop(dns::BLOCKY_SERVICE, Scope::System);
    let _ = systemd_units::systemd_disable(&[dns::BLOCKY_SERVICE], Scope::System);
}

/// Returns true if blocky encrypted DNS proxy is currently active.
pub fn is_blocky_active() -> bool {
    systemd_units::systemd_is_active(dns::BLOCKY_SERVICE, Scope::System).unwrap_or(false)
}

pub fn remove_dblock(dialog_tx: Sender<DialogMessage>) {
    if Path::new("/var/lib/pacman/db.lck").exists() {
        let _ = utils::pkexec_cmd(&["rm", "/var/lib/pacman/db.lck"]);
        if !Path::new("/var/lib/pacman/db.lck").exists() {
            dialog_tx
                .send_blocking(DialogMessage {
                    msg: fl!("removed-db-lock"),
                    msg_type: MessageType::Info,
                    action: Action::RemoveLock,
                })
                .expect("Couldn't send data to channel");
        }
    } else {
        dialog_tx
            .send_blocking(DialogMessage {
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
    let mut orphan_pkgs = utils::cmd_output("/sbin/pacman", &["-Qtdq"]);

    // get list of packages separated by space,
    // and check if it's empty or not.
    orphan_pkgs = orphan_pkgs.replace('\n', " ");
    if orphan_pkgs.is_empty() {
        dialog_tx
            .send_blocking(DialogMessage {
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
            .send_blocking(DialogMessage {
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

pub fn install_vram_management(callback: RunCmdCallback, dialog_tx: Sender<DialogMessage>) {
    let mut packages: Vec<&str> = vec!["dmemcg-booster"];
    if utils::is_kwin_wayland() {
        packages.push("plasma-foreground-booster");
    }
    install_needed_packages(
        callback,
        &packages,
        fl!("vram-management-package-installed"),
        Action::InstallVramManagement,
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
        let result = systemd_units::systemd_enable(&[DOCKER_TARGET], Scope::System, false);
        if result.is_err() {
            dialog_tx
                .send_blocking(DialogMessage {
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
    if utils::is_alpm_pkg_installed("docker")
        && !group_added
        && let Ok(current_user) = env::var("USER")
    {
        let failed = utils::pkexec_cmd(&["/sbin/usermod", "-aG", "docker", &current_user])
            .map_or(true, |s| !s.success());
        if failed {
            dialog_tx
                .send_blocking(DialogMessage {
                    msg: fl!("winboat-install-failed"),
                    msg_type: MessageType::Error,
                    action: Action::InstallWinboat,
                })
                .expect("Couldn't send data to channel");
        }
    }
}
