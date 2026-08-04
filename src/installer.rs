use crate::gui::Gui;
use crate::ui::{MessageType, UI};
use crate::{G_HELLO_WINDOW, check_regular_file, fl};

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use gtk::prelude::{BuilderExtManual, WidgetExt};

use serde::Deserialize;
use subprocess::{Exec, Redirection};
use tracing::{error, info};

#[derive(Deserialize)]
struct Versions {
    #[serde(rename = "desktopISOVersion")]
    desktop_iso_version: String,
    #[serde(rename = "handheldISOVersion")]
    handheld_iso_version: String,
}

fn outdated_version_check(ui: &Gui, message: String) -> bool {
    let edition_tag: String =
        fs::read_to_string("/etc/edition-tag").unwrap_or("desktop".into()).trim().into();
    let version_tag: String =
        fs::read_to_string("/etc/version-tag").unwrap_or("testing".into()).trim().into();

    if version_tag.contains("testing") {
        ui.show_message(MessageType::Warning, &fl!("testing-iso-warning"), message.clone());
        return true;
    }

    let response = reqwest::blocking::get("https://cachyos.org/versions.json");
    if response.is_err() {
        ui.show_message(MessageType::Warning, &fl!("offline-error"), message.clone());
        return false;
    }

    // silently continue in case of server error
    let versions = response.map(|x| x.json::<Versions>().unwrap());
    if let Err(vers_err) = versions {
        error!("Failed to fetch versions.json: {vers_err}");
        return true;
    }

    let latest_version = if edition_tag.contains("desktop") {
        versions.unwrap().desktop_iso_version
    } else {
        versions.unwrap().handheld_iso_version
    }
    .trim()
    .to_owned();

    // in most cases it should be just date number (YYMMDD)
    let parsed_ver = version_tag.parse::<u32>();
    let parsed_latestver = latest_version.parse::<u32>();
    if parsed_ver.is_ok()
        && parsed_latestver.is_ok()
        && parsed_ver.unwrap() > parsed_latestver.unwrap()
    {
        ui.show_message(MessageType::Warning, &fl!("testing-iso-warning"), message.clone());
        return true;
    }

    if version_tag != latest_version {
        ui.show_message(MessageType::Warning, &fl!("outdated-version-warning"), message.clone());
    }
    true
}

fn edition_compat_check(ui: &Gui, message: String) -> bool {
    let edition_tag = fs::read_to_string("/etc/edition-tag").unwrap_or("desktop".to_string());

    let profiles_path = format!("{}/handhelds/profiles.toml", chwd::consts::CHWD_PCI_CONFIG_DIR);

    let handheld_profiles =
        chwd::profile::parse_profiles(&profiles_path).expect("Failed to parse profiles");
    let handheld_profile_names: Vec<_> =
        handheld_profiles.iter().map(|profile| &profile.name).collect();

    let available_profiles = chwd::profile::get_available_profiles(false);
    let supported_handheld =
        available_profiles.iter().any(|profile| handheld_profile_names.contains(&&profile.name));
    if edition_tag == "handheld" && !supported_handheld {
        ui.show_message(MessageType::Warning, &fl!("unsupported-hw-warning"), message.clone());
        return false;
    } else if edition_tag == "desktop" && supported_handheld {
        ui.show_message(MessageType::Error, &fl!("desktop-on-handheld-error"), message.clone());
        return false;
    }
    true
}

fn connectivity_check(ui: &Gui, message: String) -> bool {
    // First try HTTP check to cachyos.org
    let http_status = match reqwest::blocking::get("https://cachyos.org") {
        Ok(resp) => resp.status().is_success() || resp.status().is_server_error(),
        _ => false,
    };

    if http_status {
        return true;
    }

    // If HTTP check fails, try ping fallback to reliable DNS servers
    let targets = [
        "8.8.8.8",
        "1.1.1.1",
        "9.9.9.9",
        "2001:4860:4860::8888",
        "2606:4700:4700::1111",
        "2620:fe::fe",
    ];
    for target in targets {
        let ping_result = Exec::cmd("/sbin/ping").args(["-c", "1", "-W", "3", target]).join();
        if ping_result.is_ok_and(|status: subprocess::ExitStatus| status.success()) {
            info!("Connectivity confirmed via ping to {target}");
            return true;
        }
    }

    // All connectivity checks failed
    ui.show_message(MessageType::Error, &fl!("offline-error"), message);
    false
}

pub fn launch_installer(message: String) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let window_ref = &G_HELLO_WINDOW.get().unwrap().window;
        let builder = &G_HELLO_WINDOW.get().unwrap().builder;

        let install_btn: gtk::Button = builder.object("install").unwrap();
        install_btn.set_sensitive(false);

        let ui_comp = crate::gui::Gui::new(window_ref.clone());
        let checks = [connectivity_check, edition_compat_check, outdated_version_check];
        if !checks.iter().all(|x| x(&ui_comp, message.clone())) {
            // if any check failed, return
            info!("Some ISO check failed!");
            install_btn.set_sensitive(true);
            return;
        }

        // Spawning child process
        info!("ISO checks passed! Starting Installer..");
        let mut child = Exec::cmd("/usr/local/bin/calamares-online.sh")
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .start()
            .expect("Failed to spawn installer");

        let child_out = child.stdout.take().unwrap();
        let child_read = BufReader::new(child_out);

        // Read the output line by line until EOF
        for line_result in child_read.lines() {
            match line_result {
                Ok(line) => info!("{line}"),
                Err(e) => error!("Error reading output: {e}"),
            }
        }

        let status = child.wait().expect("Failed to waiting for child");
        info!("Installer finished with status: {:?}", status);

        install_btn.set_sensitive(true);
    });
}

pub fn is_iso(preferences: &serde_json::Value) -> bool {
    Path::new(&preferences["live_path"].as_str().unwrap()).exists()
        && check_regular_file(preferences["installer_path"].as_str().unwrap())
}
