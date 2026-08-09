use crate::gui::Gui;
use crate::ui::{MessageType, UI};
use crate::{G_HELLO_WINDOW, check_regular_file, fl};

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

use gtk::glib;
use gtk::prelude::{BuilderExtManual, WidgetExt};

use serde::Deserialize;
use subprocess::{Exec, Redirection};
use tracing::{error, info};

/// HTTP checks must not block forever.
const HTTP_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Deserialize)]
struct Versions {
    #[serde(rename = "desktopISOVersion")]
    desktop_iso_version: String,
    #[serde(rename = "handheldISOVersion")]
    handheld_iso_version: String,
}

enum CheckOutcome {
    /// Passed silently
    Pass,
    /// Warn the user
    Warn(String),
    /// Show a message and abort
    Fail(MessageType, String),
}

enum InstallerMsg {
    Dialog(MessageType, String),
    ChecksDone { proceed: bool },
    Finished,
}

/// Blocking HTTP GET with a bounded timeout.
fn http_get(url: &str) -> reqwest::Result<reqwest::blocking::Response> {
    reqwest::blocking::Client::builder().timeout(HTTP_TIMEOUT).build()?.get(url).send()
}

fn outdated_version_check() -> CheckOutcome {
    let edition_tag: String =
        fs::read_to_string("/etc/edition-tag").unwrap_or("desktop".into()).trim().into();
    let version_tag: String =
        fs::read_to_string("/etc/version-tag").unwrap_or("testing".into()).trim().into();

    if version_tag.contains("testing") {
        return CheckOutcome::Warn(fl!("testing-iso-warning"));
    }

    let response = http_get("https://cachyos.org/versions.json");
    if response.is_err() {
        return CheckOutcome::Fail(MessageType::Warning, fl!("offline-error"));
    }

    // silently continue in case of server error
    let versions = response.and_then(|x| x.json::<Versions>());
    if let Err(vers_err) = versions {
        error!("Failed to fetch versions.json: {vers_err}");
        return CheckOutcome::Pass;
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
        return CheckOutcome::Warn(fl!("testing-iso-warning"));
    }

    if version_tag != latest_version {
        return CheckOutcome::Warn(fl!("outdated-version-warning"));
    }
    CheckOutcome::Pass
}

fn edition_compat_check() -> CheckOutcome {
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
        return CheckOutcome::Fail(MessageType::Warning, fl!("unsupported-hw-warning"));
    } else if edition_tag == "desktop" && supported_handheld {
        return CheckOutcome::Fail(MessageType::Error, fl!("desktop-on-handheld-error"));
    }
    CheckOutcome::Pass
}

fn connectivity_check() -> CheckOutcome {
    // First try HTTP check to cachyos.org
    let http_status = match http_get("https://cachyos.org") {
        Ok(resp) => resp.status().is_success() || resp.status().is_server_error(),
        _ => false,
    };

    if http_status {
        return CheckOutcome::Pass;
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
            return CheckOutcome::Pass;
        }
    }

    // All connectivity checks failed
    CheckOutcome::Fail(MessageType::Error, fl!("offline-error"))
}

fn run_checks(tx: &async_channel::Sender<InstallerMsg>) {
    let checks: [fn() -> CheckOutcome; 3] =
        [connectivity_check, edition_compat_check, outdated_version_check];

    for check in checks {
        match check() {
            CheckOutcome::Pass => {},
            CheckOutcome::Warn(body) => {
                let _ = tx.send_blocking(InstallerMsg::Dialog(MessageType::Warning, body));
            },
            CheckOutcome::Fail(msg_type, body) => {
                let _ = tx.send_blocking(InstallerMsg::Dialog(msg_type, body));
                let _ = tx.send_blocking(InstallerMsg::ChecksDone { proceed: false });
                return;
            },
        }
    }
    let _ = tx.send_blocking(InstallerMsg::ChecksDone { proceed: true });
}

fn run_installer_process() {
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
}

pub fn launch_installer(message: String) {
    let builder = &G_HELLO_WINDOW.get().unwrap().builder;
    let install_btn: gtk::Button = builder.object("install").unwrap();
    install_btn.set_sensitive(false);

    let (tx, rx) = async_channel::unbounded::<InstallerMsg>();
    let checks_tx = tx.clone();
    std::thread::spawn(move || {
        run_checks(&checks_tx);
    });

    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = rx.recv().await {
            match msg {
                InstallerMsg::Dialog(msg_type, body) => {
                    let window = G_HELLO_WINDOW.get().unwrap().window.clone();
                    Gui::new(window).show_message(msg_type, &body, message.clone());
                },
                InstallerMsg::ChecksDone { proceed } => {
                    if !proceed {
                        info!("Some ISO check failed!");
                        install_btn.set_sensitive(true);
                        break;
                    }

                    info!("ISO checks passed! Starting Installer..");
                    let done_tx = tx.clone();
                    std::thread::spawn(move || {
                        run_installer_process();
                        let _ = done_tx.send_blocking(InstallerMsg::Finished);
                    });
                },
                InstallerMsg::Finished => {
                    install_btn.set_sensitive(true);
                    break;
                },
            }
        }
    });
}

pub fn is_iso(preferences: &serde_json::Value) -> bool {
    Path::new(&preferences["live_path"].as_str().unwrap()).exists()
        && check_regular_file(preferences["installer_path"].as_str().unwrap())
}
