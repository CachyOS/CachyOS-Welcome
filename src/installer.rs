use crate::gui::GUI;
use crate::ui::{MessageType, UI};
use crate::{check_regular_file, fl, G_HELLO_WINDOW};

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use gtk::prelude::*;

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

fn outdated_version_check(ui: &GUI, message: String) -> bool {
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

fn edition_compat_check(ui: &GUI, message: String) -> bool {
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

fn connectivity_check(ui: &GUI, message: String) -> bool {
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
        let ping_result = Exec::cmd("/sbin/ping").args(&["-c", "1", "-W", "3", target]).join();
        if ping_result.is_ok_and(subprocess::ExitStatus::success) {
            info!("Connectivity confirmed via ping to {target}");
            return true;
        }
    }

    // All connectivity checks failed
    ui.show_message(MessageType::Error, &fl!("offline-error"), message);
    false
}

fn installer_state_label_key(busy: bool) -> &'static str {
    if busy {
        "button-installer-wait-label"
    } else {
        "button-installer-label"
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InstallerButtonState {
    sensitive: bool,
    spinner_active: bool,
    spinner_visible: bool,
}

fn installer_button_state(busy: bool) -> InstallerButtonState {
    InstallerButtonState {
        sensitive: !busy,
        spinner_active: busy,
        spinner_visible: busy,
    }
}

fn installer_state_label_text(busy: bool) -> String {
    crate::localization::get_locale_text(installer_state_label_key(busy))
}

fn installer_widgets(builder: &gtk::Builder) -> (gtk::Button, gtk::Spinner, gtk::Label) {
    let button: gtk::Button = builder.object("install").unwrap();
    let spinner: gtk::Spinner = builder.object("install-spinner").unwrap();
    let label: gtk::Label = builder.object("install-label").unwrap();

    (button, spinner, label)
}

fn installer_is_busy_state(button_sensitive: bool, spinner_active: bool) -> bool {
    !button_sensitive || spinner_active
}

fn installer_is_busy(button: &gtk::Button, spinner: &gtk::Spinner) -> bool {
    installer_is_busy_state(button.is_sensitive(), spinner.is_active())
}

fn apply_installer_busy_state(
    button: &gtk::Button,
    spinner: &gtk::Spinner,
    label: &gtk::Label,
    busy: bool,
) {
    let state = installer_button_state(busy);

    button.set_sensitive(state.sensitive);
    spinner.set_active(state.spinner_active);
    if state.spinner_visible {
        spinner.show();
    } else {
        spinner.hide();
    }

    label.set_text(&installer_state_label_text(busy));
}

pub fn refresh_installer_label(builder: &gtk::Builder) {
    let (button, spinner, label) = installer_widgets(builder);
    label.set_text(&installer_state_label_text(installer_is_busy(&button, &spinner)));
}

fn set_installer_busy(
    button: gtk::glib::SendWeakRef<gtk::Button>,
    spinner: gtk::glib::SendWeakRef<gtk::Spinner>,
    label: gtk::glib::SendWeakRef<gtk::Label>,
    busy: bool,
) {
    gtk::glib::MainContext::default().invoke(move || {
        let (Some(button), Some(spinner), Some(label)) =
            (button.upgrade(), spinner.upgrade(), label.upgrade())
        else {
            return;
        };

        apply_installer_busy_state(&button, &spinner, &label, busy);
    });
}

pub fn launch_installer(message: String) {
    let builder = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().builder };
    let (button, spinner, label) = installer_widgets(builder);
    let button_ref: gtk::glib::SendWeakRef<gtk::Button> = button.downgrade().into();
    let spinner_ref: gtk::glib::SendWeakRef<gtk::Spinner> = spinner.downgrade().into();
    let label_ref: gtk::glib::SendWeakRef<gtk::Label> = label.downgrade().into();
    apply_installer_busy_state(&button, &spinner, &label, true);

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let window_ref = unsafe { G_HELLO_WINDOW.as_ref().unwrap().window.clone() };
        let ui_comp = crate::gui::GUI::new(window_ref);
        let checks = [connectivity_check, edition_compat_check, outdated_version_check];
        if !checks.iter().all(|x| x(&ui_comp, message.clone())) {
            // if any check failed, return
            info!("Some ISO check failed!");
            set_installer_busy(button_ref.clone(), spinner_ref.clone(), label_ref.clone(), false);
            return;
        }

        // Spawning child process
        info!("ISO checks passed! Starting Installer..");
        let mut child = match Exec::cmd("/usr/local/bin/calamares-online.sh")
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .popen()
        {
            Ok(child) => child,
            Err(err) => {
                error!("Failed to spawn installer: {err}");
                set_installer_busy(
                    button_ref.clone(),
                    spinner_ref.clone(),
                    label_ref.clone(),
                    false,
                );
                return;
            },
        };

        let Some(child_out) = child.stdout.take() else {
            error!("Failed to capture installer stdout");
            set_installer_busy(button_ref.clone(), spinner_ref.clone(), label_ref.clone(), false);
            return;
        };
        let child_read = BufReader::new(child_out);

        // Read the output line by line until EOF
        for line_result in child_read.lines() {
            match line_result {
                Ok(line) => info!("{line}"),
                Err(e) => error!("Error reading output: {e}"),
            }
        }

        match child.wait() {
            Ok(status) => info!("Installer finished with status: {:?}", status),
            Err(err) => error!("Failed waiting for installer: {err}"),
        }

        set_installer_busy(button_ref, spinner_ref, label_ref, false);
    });
}

pub fn is_iso(preferences: &serde_json::Value) -> bool {
    Path::new(&preferences["live_path"].as_str().unwrap()).exists()
        && check_regular_file(preferences["installer_path"].as_str().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Mutex, OnceLock};
    use unic_langid::LanguageIdentifier;

    fn locale_test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn installer_button_state_matches_idle_ui() {
        let state = installer_button_state(false);

        assert_eq!(
            state,
            InstallerButtonState {
                sensitive: true,
                spinner_active: false,
                spinner_visible: false,
            }
        );
    }

    #[test]
    fn installer_button_state_matches_busy_ui() {
        let state = installer_button_state(true);

        assert_eq!(
            state,
            InstallerButtonState {
                sensitive: false,
                spinner_active: true,
                spinner_visible: true,
            }
        );
    }

    #[test]
    fn locale_refresh_busy_detection_matches_button_and_spinner_state() {
        assert!(!installer_is_busy_state(true, false));
        assert!(installer_is_busy_state(false, false));
        assert!(installer_is_busy_state(true, true));
    }

    #[test]
    fn installer_busy_label_uses_selected_locale() {
        let _guard = locale_test_lock().lock().unwrap();
        let localizer = crate::localization::localizer();
        let german: LanguageIdentifier = "de".parse().unwrap();
        let english: LanguageIdentifier = "en".parse().unwrap();

        localizer.select(&[german]).unwrap();
        assert_eq!(installer_state_label_text(true), "Installation starten…");

        localizer.select(&[english]).unwrap();
        assert_eq!(installer_state_label_text(true), "Launching installer…");
    }

    #[test]
    fn installer_button_glade_contains_spinner_and_embedded_label() {
        let glade = include_str!("../ui/cachyos-hello.glade");

        assert!(glade.contains(r#"<object class="GtkSpinner" id="install-spinner">"#));
        assert!(glade.contains(r#"<object class="GtkLabel" id="install-label">"#));
    }

    #[test]
    fn all_locales_define_busy_installer_label() {
        let mut missing = std::fs::read_dir("i18n")
            .unwrap()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path().join("cachyos_hello.ftl"))
            .filter(|path| path.exists())
            .filter_map(|path| {
                let file = std::fs::read_to_string(&path).unwrap();
                if file.contains("button-installer-wait-label =") {
                    None
                } else {
                    Some(path)
                }
            })
            .collect::<Vec<_>>();

        missing.sort();
        assert!(missing.is_empty(), "missing busy label in: {missing:?}");
    }
}
