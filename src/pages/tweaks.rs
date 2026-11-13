use crate::tweak::{self, TweakName};
use crate::ui::{MessageType, UI};
use crate::{actions, fl, systemd_units, utils};

use std::str;

use gtk::prelude::*;

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

// logind limits /run/user to 10% of RAM and psd keeps browser profiles there
// Word joiners keep the path from being wrapped at its slashes in the dialog
const PSD_RUNTIME_DIR: &str = "/\u{2060}run/\u{2060}user";
const PSD_LOW_MEMORY_KIB: u64 = 8 * 1024 * 1024;

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

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        if !actions::toggle_tweak(tweak_name, !action_enabled, crate::gui::run_command) {
            tx.send_blocking(false).expect("Couldn't send data to channel");
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

    if matches!(tweak_name, TweakName::Psd)
        && button.is_active()
        && let Some(ram_gib) = low_memory_gib()
        && !confirm_psd_low_memory(&widget_window, ram_gib)
    {
        let sighandle_id_obj = unsafe { glib::signal::SignalHandlerId::from_glib(signal_handler) };
        button.block_signal(&sighandle_id_obj);
        button.set_active(false);
        button.unblock_signal(&sighandle_id_obj);
        return;
    }

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

/// Returns the total RAM rounded up to whole GiB if it is at or below the psd threshold.
fn low_memory_gib() -> Option<u64> {
    utils::get_total_memory_kib()
        .filter(|&kib| kib <= PSD_LOW_MEMORY_KIB)
        .map(|kib| kib.div_ceil(1024 * 1024))
}

fn confirm_psd_low_memory(window: &gtk::Window, ram_gib: u64) -> bool {
    let dialog = gtk::MessageDialog::builder()
        .transient_for(window)
        .message_type(gtk::MessageType::Warning)
        .text(fl!("tweak-psd-low-memory-warning", ram = ram_gib, path = PSD_RUNTIME_DIR))
        .title(MessageType::Warning.to_string())
        .modal(true)
        .buttons(gtk::ButtonsType::YesNo)
        .build();
    dialog.show();
    let response = dialog.run();
    dialog.close();
    response == gtk::ResponseType::Yes
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
