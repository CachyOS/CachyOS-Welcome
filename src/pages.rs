// NOTE: This entire file needs refactoring.
// GTK widget creation is replaced by QML.
// Data fetching and action logic (systemd, pacman, nmcli calls)
// must be moved into the AppModel struct in main.rs (or model.rs).

/* // Comment out entire GTK-based implementation

use crate::application_browser::ApplicationBrowser;
use crate::systemd_units::SystemdUnits;
use crate::utils::PacmanWrapper;
use crate::{fl, kwin_dbus, systemd_units, utils};

use std::boxed::Box;
use std::fmt::Write;
use std::path::Path;
use std::str;
use std::sync::Mutex;

use glib::translate::FromGlib;
use gtk::{glib, Builder};
use gtk::prelude::*;
use once_cell::sync::Lazy;
use phf::phf_ordered_map;
use subprocess::{Exec, Redirection};
use tokio::runtime::Runtime;
use tracing::{debug, error};
use which::which;

use crate::{config, utils, RESPREFIX};
use crate::config::APP_ID;
use crate::localization::localizer;

// Macros like create_gtk_button, create_tweak_checkbox are no longer needed.

// Static data like G_DNS_SERVERS might still be useful in AppModel.
static G_DNS_SERVERS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "Adguard" => "94.140.14.14",
    "Adguard Family Protection" => "94.140.14.15",
    "Cloudflare" => "1.1.1.1",
    "Cloudflare Malware and adult content blocking" => "1.1.1.3",
    "DNS.Watch" => "84.200.69.80",
    "Cisco Umbrella(OpenDNS)" => "208.67.222.222,208.67.220.220",
    "Quad9" => "9.9.9.9",
    "Google" => "8.8.8.8,8.8.4.4",
    "Yandex" => "77.88.8.8,77.88.8.1",
};

// Systemd unit handling logic (G_LOCAL_UNITS, G_GLOBAL_UNITS, load_enabled_units)
// needs to be integrated into AppModel state/methods.

// DialogMessage enum and associated channel logic replaced by signals/methods/QML dialogs.

// update_translation_* functions replaced by QML bindings to AppModel properties.

// set_tweak_check_data, connect_tweak replaced by QML property bindings and AppModel methods.

// get_nm_connections needs to be called from an AppModel method.

// launch_kwin_debug_window needs to be called from an AppModel method.

// create_fixes_section: Logic inside (running commands, checking paths) moves to AppModel methods.

// create_options_section: Logic inside (checking service status, toggling) moves to AppModel methods.

// create_apps_section: Logic moves to AppModel methods.

// create_connections_section: Logic (getting connections, setting DNS) moves to AppModel methods.

// load_enabled_units, load_global_enabled_units: Call from AppModel initialization or refresh method.

// create_tweaks_page, create_dnsconnections_page, create_appbrowser_page:
// These functions are entirely replaced by QML page components and AppModel providing data/actions.

// toggle_service: Logic moves to an AppModel method.

// on_* callbacks (on_servbtn_clicked, on_refreshkeyring_btn_clicked, etc.):
// Replaced by AppModel methods invoked from QML onClicked handlers.

pub fn create_tweaks_page(builder: &Builder) {
    let install: gtk::Button = builder.object("tweaksBrowser").unwrap();
    install.set_visible(true);
    install.set_label(&fl!("tweaksbrowser-label"));

    load_enabled_units();
    load_global_enabled_units();

    let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    let image = gtk::Image::from_icon_name(Some("go-previous"), gtk::IconSize::Button);
    let back_btn = gtk::Button::new();
    back_btn.set_image(Some(&image));
    back_btn.set_widget_name("home");

    back_btn.connect_clicked(glib::clone!(@weak builder => move |button| {
        let name = button.widget_name();
        let stack: gtk::Stack = builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{name}page"));
    }));

    let options_section_box = create_options_section();
    let fixes_section_box = create_fixes_section(builder);
    let apps_section_box_opt = create_apps_section();

    let child_name = "tweaksBrowserpage";
    options_section_box.set_widget_name(&format!("{child_name}_options"));
    fixes_section_box.set_widget_name(&format!("{child_name}_fixes"));
    if apps_section_box_opt.is_some() {
        apps_section_box_opt.as_ref().unwrap().set_widget_name(&format!("{child_name}_apps"));
    }

    let grid = gtk::Grid::new();
    grid.set_hexpand(true);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_margin_top(5);
    grid.set_margin_bottom(5);
    grid.attach(&back_btn, 0, 1, 1, 1);
    let box_collection_s = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let box_collection = gtk::Box::new(gtk::Orientation::Vertical, 5);
    box_collection.set_widget_name(child_name);

    box_collection.pack_start(&options_section_box, false, false, 10);
    box_collection.pack_start(&fixes_section_box, false, false, 10);

    if let Some(apps_section_box) = apps_section_box_opt {
        box_collection.pack_end(&apps_section_box, false, false, 10);
    }

    box_collection.set_valign(gtk::Align::Center);
    box_collection.set_halign(gtk::Align::Center);
    box_collection_s.pack_start(&grid, false, false, 0);
    box_collection_s.pack_start(&box_collection, false, false, 10);
    viewport.add(&box_collection_s);
    viewport.show_all();

    let stack: gtk::Stack = builder.object("stack").unwrap();
    stack.add_named(&viewport, child_name);
}

pub fn create_dnsconnections_page(builder: &Builder) {
    let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    let image = gtk::Image::from_icon_name(Some("go-previous"), gtk::IconSize::Button);
    let back_btn = gtk::Button::new();
    back_btn.set_image(Some(&image));
    back_btn.set_widget_name("tweaksBrowser");

    back_btn.connect_clicked(glib::clone!(@weak builder => move |button| {
        let name = button.widget_name();
        let stack: gtk::Stack = builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{name}page"));
    }));

    let connections_section_box = create_connections_section();

    let child_name = "dnsConnectionsBrowserpage";
    connections_section_box.set_widget_name(&format!("{child_name}_connections"));

    let grid = gtk::Grid::new();
    grid.set_hexpand(true);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_margin_top(5);
    grid.set_margin_bottom(5);
    grid.attach(&back_btn, 0, 1, 1, 1);
    let box_collection_s = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let box_collection = gtk::Box::new(gtk::Orientation::Vertical, 5);
    box_collection.set_widget_name(child_name);

    box_collection.pack_start(&connections_section_box, false, false, 10);

    box_collection.set_valign(gtk::Align::Center);
    box_collection.set_halign(gtk::Align::Center);
    box_collection_s.pack_start(&grid, false, false, 0);
    box_collection_s.pack_start(&box_collection, false, false, 10);
    viewport.add(&box_collection_s);
    viewport.show_all();

    let stack: gtk::Stack = builder.object("stack").unwrap();
    stack.add_named(&viewport, child_name);
}

pub fn create_appbrowser_page(builder: &Builder) {
    let install: gtk::Button = builder.object("appBrowser").unwrap();
    install.set_visible(true);
    install.set_label(&fl!("appbrowser-label"));

    let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    let back_btn = ApplicationBrowser::back_btn_impl()
        .expect("Failed to get back btn from application browser");
    back_btn.connect_clicked(glib::clone!(@weak builder => move |button| {
        let name = button.widget_name();
        let stack: gtk::Stack = builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{name}page"));
    }));
    let app_browser_box =
        ApplicationBrowser::page_impl().expect("Failed to get page of application browser");

    // Add grid to the viewport
    // NOTE: we might eliminate that?
    viewport.add(&app_browser_box);
    viewport.show_all();

    let stack: gtk::Stack = builder.object("stack").unwrap();
    let child_name = "appBrowserpage";
    stack.add_named(&viewport, child_name);
}

fn toggle_service(
    action_type: &str,
    action_data: &str,
    alpm_package_name: &str,
    widget_window: gtk::Window,
    callback: std::boxed::Box<dyn Fn(bool)>,
) {
    let units_handle = if action_type == "user_service" { &G_GLOBAL_UNITS } else { &G_LOCAL_UNITS }
        .lock()
        .unwrap();
    let cmd = if !units_handle.enabled_units.contains(&String::from(action_data)) {
        if action_type == "user_service" {
            format!("systemctl --user enable --now --force {action_data}")
        } else {
            format!("/sbin/pkexec bash -c \"systemctl enable --now --force {action_data}\"")
        }
    } else if action_type == "user_service" {
        format!("systemctl --user disable --now {action_data}")
    } else {
        format!("/sbin/pkexec bash -c \"systemctl disable --now {action_data}\"")
    };

    // Create context channel.
    let (tx, rx) = glib::MainContext::channel(glib::Priority::default());

    let dialog_text = fl!("package-not-installed", package_name = alpm_package_name);

    let action_type = action_type.to_owned();
    let alpm_package_name = alpm_package_name.to_owned();
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        if !alpm_package_name.is_empty() {
            if !utils::is_alpm_pkg_installed(&alpm_package_name) {
                let _ = utils::run_cmd_terminal(format!("pacman -S {alpm_package_name}"), true);
            }
            if !utils::is_alpm_pkg_installed(&alpm_package_name) {
                tx.send(false).expect("Couldn't send data to channel");
                return;
            }
        }
        Exec::shell(cmd).join().unwrap();

        if action_type == "user_service" {
            load_global_enabled_units();
        } else {
            load_enabled_units();
        }
    });

    rx.attach(None, move |msg| {
        if !msg {
            callback(msg);

            utils::show_simple_dialog(
                &widget_window,
                gtk::MessageType::Error,
                &dialog_text,
                "Error".to_string(),
            );
        }
        glib::ControlFlow::Continue
    });
}

fn on_servbtn_clicked(button: &gtk::CheckButton) {
    // Get action data/type.
    let action_type: &str;
    let action_data: &str;
    let alpm_package_name: &str;
    let signal_handler: u64;
    unsafe {
        action_type = *button.data("actionType").unwrap().as_ptr();
        action_data = *button.data("actionData").unwrap().as_ptr();
        alpm_package_name = *button.data("alpmPackage").unwrap().as_ptr();
        signal_handler = *button.data("signalHandle").unwrap().as_ptr();
    }

    let widget_window = utils::get_window_from_widget(button).expect("Failed to retrieve window");

    let button_sh = button.clone();
    toggle_service(
        action_type,
        action_data,
        alpm_package_name,
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

fn on_refreshkeyring_btn_clicked(_: &gtk::Button) {
    let pacman = pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
    let alpm = alpm_utils::alpm_with_conf(&pacman).unwrap();

    // search local database for packages matching the regex ".*-keyring"
    // e.g pacman -Qq | grep keyring
    let needles: &[String] = &[".*-keyring".into()];
    let found_keyrings = alpm
        .localdb()
        .search(needles.iter())
        .unwrap()
        .into_iter()
        .filter(|pkg| pkg.name() != "gnome-keyring" && pkg.name() != "python-keyring")
        .fold(String::new(), |mut output, pkg| {
            let pkgname = str::replace(pkg.name(), "-keyring", "");
            let _ = write!(output, "{pkgname} ");
            output
        });

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let _ = utils::run_cmd_terminal(
            format!("pacman-key --init && pacman-key --populate {found_keyrings}"),
            true,
        );
    });
}

fn on_update_system_btn_clicked(_: &gtk::Button) {
    let (cmd, escalate) = match utils::get_pacman_wrapper() {
        PacmanWrapper::Aura => ("aura -Syu && aura -Akaxu", false),
        _ => ("pacman -Syu", true),
    };
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let _ = utils::run_cmd_terminal(String::from(cmd), escalate);
    });
}

fn on_clear_pkgcache_btn_clicked(_: &gtk::Button) {
    let (cmd, escalate) = match utils::get_pacman_wrapper() {
        PacmanWrapper::Pak => ("pak -Sc", false),
        PacmanWrapper::Yay => ("yay -Sc", false),
        PacmanWrapper::Paru => ("paru -Sc", false),
        _ => ("pacman -Sc", true),
    };
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let _ = utils::run_cmd_terminal(String::from(cmd), escalate);
    });
}

fn on_appbtn_clicked(button: &gtk::Button) {
    // Get button label.
    let name = button.label().unwrap();
    let binname = if name == "CachyOS PackageInstaller" {
        "cachyos-pi"
    } else if name == "CachyOS Kernel Manager" {
        "cachyos-kernel-manager"
    } else {
        ""
    };

    // Get executable path, overwise return if it doesn't exist.
    let exec_path = which(binname);
    if exec_path.is_err() {
        return;
    }

    // Create context channel.
    let (tx, rx) = glib::MainContext::channel(glib::Priority::default());

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        // Get executable path.
        let exec_path = exec_path.unwrap().to_str().unwrap().to_owned();
        let exit_status = Exec::cmd(exec_path).detached().join().expect("Failed to spawn process");

        tx.send(format!("Exit status successfully? = {:?}", exit_status.success()))
            .expect("Couldn't send data to channel");
    });

    rx.attach(None, move |text| {
        debug!("{text}");
        glib::ControlFlow::Continue
    });
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

*/ // End comment out of GTK implementation
