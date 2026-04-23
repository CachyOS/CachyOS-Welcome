use crate::systemd_units::Scope;
use crate::tweak::{self, TweakName};
use crate::ui::{MessageType, UI};
use crate::{fl, systemd_units, utils};

use std::str;

use gtk::prelude::*;
use subprocess::Exec;

use glib::translate::FromGlib;
use gtk::glib;

#[macro_export]
macro_rules! create_tweak_checkbox {
    ($tweak_msg:literal,$tweak_name:expr) => {{
        let temp_btn =
            gtk::CheckButton::with_label(&fl!("tweak-enabled-title", tweak = $tweak_msg));
        temp_btn.set_widget_name($tweak_msg);

        set_tweak_check_data(&temp_btn, $tweak_name);

        let (_, action_data, _) = tweak::get_details($tweak_name);
        connect_tweak(&temp_btn, $tweak_name, action_data);
        temp_btn
    }};
}

fn set_tweak_check_data(check_btn: &gtk::CheckButton, tweak_name: TweakName) {
    unsafe {
        check_btn.set_data("tweakName", tweak_name);
    }
}

fn connect_tweak(check_btn: &gtk::CheckButton, tweak_name: TweakName, action_data: &'static str) {
    let is_active =
        systemd_units::check_any_units(action_data) || tweak::check_autostart_active(tweak_name);
    check_btn.set_active(is_active);

    connect_clicked_and_save(check_btn, on_servbtn_clicked);
}

pub(crate) fn create_options_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let box_collection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let box_collection_s = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("tweaks"));

    let psd_btn = create_tweak_checkbox!("Profile-sync-daemon", TweakName::Psd);
    let systemd_oomd_btn = create_tweak_checkbox!("Systemd-oomd", TweakName::Oomd);
    let bpftune_btn = create_tweak_checkbox!("Bpftune", TweakName::Bpftune);
    let bluetooth_btn = create_tweak_checkbox!("Bluetooth", TweakName::Bluetooth);
    let ananicy_cpp_btn = create_tweak_checkbox!("Ananicy Cpp", TweakName::Ananicy);
    let cachy_update_btn = create_tweak_checkbox!("Cachy Update", TweakName::CachyUpdate);

    // set tooltips
    psd_btn.set_tooltip_text(Some(&fl!("tweak-psd-tooltip")));
    systemd_oomd_btn.set_tooltip_text(Some(&fl!("tweak-oomd-tooltip")));
    bpftune_btn.set_tooltip_text(Some(&fl!("tweak-bpftune-tooltip")));
    bluetooth_btn.set_tooltip_text(Some(&fl!("tweak-bluetooth-tooltip")));
    ananicy_cpp_btn.set_tooltip_text(Some(&fl!("tweak-ananicycpp-tooltip")));
    cachy_update_btn.set_tooltip_text(Some(&fl!("tweak-cachyupdate-tooltip")));

    topbox.pack_start(&label, true, false, 1);
    box_collection.pack_start(&psd_btn, true, false, 2);
    box_collection_s.pack_start(&systemd_oomd_btn, true, false, 2);
    box_collection_s.pack_start(&bpftune_btn, true, false, 2);
    box_collection.pack_start(&ananicy_cpp_btn, true, false, 2);
    box_collection.pack_start(&cachy_update_btn, true, false, 2);
    box_collection_s.pack_start(&bluetooth_btn, true, false, 2);
    box_collection.set_halign(gtk::Align::Fill);
    box_collection_s.set_halign(gtk::Align::Fill);
    topbox.pack_end(&box_collection_s, true, false, 1);
    topbox.pack_end(&box_collection, true, false, 1);

    topbox.set_hexpand(true);
    topbox
}

fn toggle_service(
    tweak_name: TweakName,
    widget_window: gtk::Window,
    callback: std::boxed::Box<dyn Fn(bool)>,
) {
    let (action_type, action_data, alpm_package_name) = tweak::get_details(tweak_name);
    let action_enabled = if action_type == "user_service" {
        systemd_units::check_user_units(action_data)
    } else {
        systemd_units::check_system_units(action_data)
    };
    // Create context channel.
    let (tx, rx) = async_channel::unbounded();

    let dialog_text = fl!("package-not-installed", package_name = alpm_package_name);

    let action_type = action_type.to_owned();
    let action_data = action_data.to_owned();
    let alpm_package_name = alpm_package_name.to_owned();
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        if !alpm_package_name.is_empty() {
            if !utils::is_alpm_pkg_installed(&alpm_package_name) {
                let _ = utils::run_cmd_terminal(
                    crate::gui::run_command,
                    format!("pacman -S {alpm_package_name}"),
                    true,
                );
            }
            if !utils::is_alpm_pkg_installed(&alpm_package_name) {
                tx.send_blocking(false).expect("Couldn't send data to channel");
                return;
            }
        }

        let scope = if action_type == "user_service" { Scope::User } else { Scope::System };
        let units: Vec<&str> = action_data.split_whitespace().collect();
        if action_enabled {
            let _ = systemd_units::systemd_disable(&units, scope);
        } else {
            let _ = systemd_units::systemd_enable(&units, scope, true);
        }

        if action_enabled && action_type == "user_service" {
            if tweak::is_globally_enabled(&action_data) {
                let _ =
                    Exec::cmd("/sbin/systemctl").args(&["--global", "disable"]).args(&units).join();
            }
            tweak::remove_autostart_files(tweak_name);
        }

        if action_type == "user_service" {
            systemd_units::refresh_user_cache();
        } else {
            systemd_units::refresh_system_cache();
        }
    });

    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = rx.recv().await {
            if !msg {
                callback(msg);

                let ui_comp = crate::gui::Gui::new(widget_window.clone());
                ui_comp.show_message(MessageType::Error, &dialog_text, "Error".to_string());
            }
        }
    });
}

fn on_servbtn_clicked(button: &gtk::CheckButton) {
    // Get action data/type.
    let tweak_name: TweakName;
    let signal_handler: u64;
    unsafe {
        tweak_name = *button.data("tweakName").unwrap().as_ptr();
        signal_handler = *button.data("signalHandle").unwrap().as_ptr();
    }

    let widget_window = utils::get_window_from_widget(button).expect("Failed to retrieve window");
    let button_sh = button.clone();

    toggle_service(
        tweak_name,
        widget_window,
        Box::new(move |msg| {
            let sighandle_id_obj =
                unsafe { glib::signal::SignalHandlerId::from_glib(signal_handler) };
            button_sh.block_signal(&sighandle_id_obj);
            button_sh.set_active(msg);
            button_sh.unblock_signal(&sighandle_id_obj);
        }),
    );
}

fn connect_clicked_and_save<F>(passed_btn: &gtk::CheckButton, callback: F)
where
    F: Fn(&gtk::CheckButton) + 'static,
{
    let sighandle_id = passed_btn.connect_clicked(callback);
    unsafe {
        passed_btn.set_data("signalHandle", sighandle_id.as_raw());
    }
}
