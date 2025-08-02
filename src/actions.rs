use crate::pages::{Action, DialogMessage};
use crate::{fl, kwin_dbus, utils, PacmanWrapper};

use std::fmt::Write;
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

pub fn launch_kwin_debug_window() {
    if let Err(kwin_err) = kwin_dbus::launch_kwin_debug_window() {
        error!("Failed to launch kwin debug window: {kwin_err}");
    }
}

pub fn change_dns_server(conn_name: &str, server_addr_ipv4: &str, server_addr_ipv6: &str, dialog_tx: Sender<DialogMessage>) {
    let status_code = Exec::cmd("/sbin/pkexec")
        .arg("bash")
        .arg("-c")
        .arg(format!(
            "nmcli con mod '{conn_name}' ipv4.dns '{server_addr_ipv4}' && \
             nmcli con mod '{conn_name}' ipv6.dns '{server_addr_ipv6}' && \
             systemctl restart NetworkManager"
        ))
        .join()
        .unwrap();
    if status_code.success() {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-changed"),
                msg_type: gtk::MessageType::Info,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    } else {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-failed"),
                msg_type: gtk::MessageType::Error,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    }
}

pub fn reset_dns_server(conn_name: &str, dialog_tx: Sender<DialogMessage>) {
    let status_code = Exec::cmd("/sbin/pkexec")
        .arg("bash")
        .arg("-c")
        .arg(format!(
            "nmcli con mod '{conn_name}' ipv4.dns '' && \
             nmcli con mod '{conn_name}' ipv6.dns '' && \
             systemctl restart NetworkManager"
        ))
        .join()
        .unwrap();
    if status_code.success() {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-reset"),
                msg_type: gtk::MessageType::Info,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    } else {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("dns-server-reset-failed"),
                msg_type: gtk::MessageType::Error,
                action: Action::SetDnsServer,
            })
            .expect("Couldn't send data to channel");
    }
}

pub fn remove_dblock(dialog_tx: Sender<DialogMessage>) {
    if Path::new("/var/lib/pacman/db.lck").exists() {
        let _ = Exec::cmd("/sbin/pkexec")
            .arg("bash")
            .arg("-c")
            .arg("rm /var/lib/pacman/db.lck")
            .join()
            .unwrap();
        if !Path::new("/var/lib/pacman/db.lck").exists() {
            dialog_tx
                .send(DialogMessage {
                    msg: fl!("removed-db-lock"),
                    msg_type: gtk::MessageType::Info,
                    action: Action::RemoveLock,
                })
                .expect("Couldn't send data to channel");
        }
    } else {
        dialog_tx
            .send(DialogMessage {
                msg: fl!("lock-doesnt-exist"),
                msg_type: gtk::MessageType::Info,
                action: Action::RemoveLock,
            })
            .expect("Couldn't send data to channel");
    }
}

pub fn update_system() {
    let (cmd, escalate) = match utils::get_pacman_wrapper() {
        PacmanWrapper::Aura => ("aura -Syu && aura -Akaxu", false),
        _ => ("pacman -Syu", true),
    };
    let _ = utils::run_cmd_terminal(String::from(cmd), escalate);
}

pub fn clear_pkgcache() {
    let (cmd, escalate) = match utils::get_pacman_wrapper() {
        PacmanWrapper::Pak => ("pak -Sc", false),
        PacmanWrapper::Yay => ("yay -Sc", false),
        PacmanWrapper::Paru => ("paru -Sc", false),
        _ => ("pacman -Sc", true),
    };
    let _ = utils::run_cmd_terminal(String::from(cmd), escalate);
}

pub fn reinstall_packages() {
    let _ = utils::run_cmd_terminal(String::from("pacman -S $(pacman -Qnq)"), true);
}

pub fn remove_orphans(dialog_tx: Sender<DialogMessage>) {
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
                msg_type: gtk::MessageType::Info,
                action: Action::RemoveOrphans,
            })
            .expect("Couldn't send data to channel");
        return;
    }
    let _ = utils::run_cmd_terminal(format!("pacman -Rns {orphan_pkgs}"), true);
}

pub fn reset_keyring() {
    let key_reset = r#"
rm -rf /etc/pacman.d/gnupg/ && \
pacman-key --init && pacman-key --populate && \
pacman-key --recv-keys F3B607488DB35A47 --keyserver keyserver.ubuntu.com && \
pacman-key --lsign-key F3B607488DB35A47
"#;

    let _ = utils::run_cmd_terminal(key_reset.into(), true);
}

pub fn install_needed_packages(
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
                msg_type: gtk::MessageType::Info,
                action: dialog_action,
            })
            .expect("Couldn't send data to channel");
        return;
    }

    // install overwise
    let packages = packages_to_install.join(" ");
    let _ = utils::run_cmd_terminal(format!("pacman -S {packages}"), true);
}
