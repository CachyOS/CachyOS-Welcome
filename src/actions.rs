use crate::ui::{Action, DialogMessage, MessageType, RunCmdCallback};
use crate::{fl, kwin_dbus, systemd_units, utils, PacmanWrapper};

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

pub fn get_dns_for_connection(conn_name: &str) -> Option<(String, String)> {
    let ips = Exec::cmd("/sbin/nmcli")
        .args(&["-g", "ipv4.dns,ipv6.dns", "con", "show", conn_name])
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    let mut lines = ips.lines();
    let ipv4_dns = lines.next().unwrap_or("").to_owned();
    let ipv6_dns = lines.next().unwrap_or("").replace("\\:", ":");

    if ipv4_dns.is_empty() && ipv6_dns.is_empty() {
        None
    } else {
        Some((ipv4_dns, ipv6_dns))
    }
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
    dialog_tx: Sender<DialogMessage>,
) {
    let status_code = utils::run_cmd(
        format!(
            "nmcli con mod '{conn_name}' ipv4.dns '{server_addr_ipv4}' && nmcli con mod \
             '{conn_name}' ipv6.dns '{server_addr_ipv6}' && systemctl restart NetworkManager"
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
    let status_code = utils::run_cmd(
        format!(
            "nmcli con mod '{conn_name}' ipv4.dns '' && nmcli con mod '{conn_name}' ipv6.dns '' \
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

pub fn install_snapper(callback: RunCmdCallback, dialog_tx: Sender<DialogMessage>) {
    install_needed_packages(
        callback,
        &["cachyos-snapper-support"],
        fl!("snapper-package-installed"),
        Action::InstallSnapper,
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

    // Enable docker.service after installation
    const DOCKER_SERVICE: &str = "docker.service";
    let docker_enabled = systemd_units::check_system_units(DOCKER_SERVICE);
    if utils::is_alpm_pkg_installed("docker") && !docker_enabled {
        let (cmd, run_as_root) =
            utils::get_tweak_toggle_cmd("service", DOCKER_SERVICE, docker_enabled);
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
}
