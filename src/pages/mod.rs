pub mod dns;
pub mod i18n;
mod tweaks;

use crate::ui::{Action, UI};
use crate::{actions, fl, systemd_units, utils};

use std::path::Path;
use std::str;

use gtk::prelude::*;

use gtk::{Builder, glib};
use tracing::debug;
use which::which;

#[macro_export]
macro_rules! create_gtk_button {
    ($message_id:literal) => {{
        let temp_btn = gtk::Button::with_label(&fl!($message_id));
        temp_btn.set_widget_name($message_id);
        temp_btn
    }};
}

fn create_utilities_section(builder: &Builder) -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let button_box_f = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_s = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_t = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("fixes"));

    let update_system_btn = create_gtk_button!("update-system-title");
    let remove_orphans_btn = create_gtk_button!("remove-orphans-title");
    let rankmirrors_btn = create_gtk_button!("rankmirrors-title");

    let install_gaming_btn = create_gtk_button!("install-gaming-title");
    let install_winboat_btn = create_gtk_button!("install-winboat-title");
    let install_vram_management_btn = utils::has_intel_or_amd_gpu().then(|| {
        let btn = create_gtk_button!("install-vram-management-title");
        btn.set_tooltip_text(Some(&fl!("install-vram-management-tooltip")));
        btn
    });

    // Create context channel.
    let (dialog_tx, dialog_rx) = async_channel::unbounded();

    // Connect signals.
    let dialog_tx_gaming = dialog_tx.clone();
    let dialog_tx_winboat = dialog_tx.clone();
    let dialog_tx_vram_management = dialog_tx.clone();
    update_system_btn.connect_clicked(on_update_system_btn_clicked);
    remove_orphans_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        let dialog_tx_clone = dialog_tx.clone();
        std::thread::spawn(move || {
            actions::remove_orphans(crate::gui::run_command, dialog_tx_clone);
        });
    });
    rankmirrors_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            actions::rankmirrors(crate::gui::run_command);
        });
    });
    install_gaming_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        let dialog_tx_gaming = dialog_tx_gaming.clone();
        std::thread::spawn(move || {
            actions::install_gaming(crate::gui::run_command, dialog_tx_gaming);
        });
    });
    install_winboat_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        let dialog_tx_winboat = dialog_tx_winboat.clone();
        std::thread::spawn(move || {
            actions::install_winboat(crate::gui::run_command, dialog_tx_winboat);
        });
    });
    if let Some(button) = &install_vram_management_btn {
        button.connect_clicked(move |_| {
            let dialog_tx_vram_management = dialog_tx_vram_management.clone();
            std::thread::spawn(move || {
                actions::install_vram_management(
                    crate::gui::run_command,
                    dialog_tx_vram_management,
                );
            });
        });
    }

    // Setup receiver.
    let remove_orphans_btn_clone = remove_orphans_btn.clone();
    let install_gaming_btn_clone = install_gaming_btn.clone();
    let install_winboat_btn_clone = install_winboat_btn.clone();
    let install_vram_management_btn_clone = install_vram_management_btn.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = dialog_rx.recv().await {
            let widget_obj = match msg.action {
                Action::RemoveOrphans => &remove_orphans_btn_clone,
                Action::InstallGaming => &install_gaming_btn_clone,
                Action::InstallWinboat => &install_winboat_btn_clone,
                Action::InstallVramManagement => install_vram_management_btn_clone
                    .as_ref()
                    .expect("VRAM management button missing"),
                _ => panic!("Unexpected action!!"),
            };
            let widget_window =
                utils::get_window_from_widget(widget_obj).expect("Failed to retrieve window");
            let ui_comp = crate::gui::Gui::new(widget_window);

            ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
        }
    });

    topbox.pack_start(&label, true, false, 1);
    button_box_f.pack_start(&update_system_btn, true, true, 2);
    button_box_f.pack_end(&remove_orphans_btn, true, true, 2);
    button_box_f.pack_end(&rankmirrors_btn, true, true, 2);
    button_box_s.pack_end(&install_gaming_btn, true, true, 2);
    button_box_s.pack_end(&install_winboat_btn, true, true, 2);
    if let Some(button) = &install_vram_management_btn {
        button_box_s.pack_end(button, true, true, 2);
    }

    if Path::new("/usr/bin/nmcli").exists() {
        let dnsserver_btn = create_gtk_button!("dnsserver-title");
        dnsserver_btn.connect_clicked(glib::clone!(@weak builder => move |_| {
            let name = "dnsConnectionsBrowser";
            let stack: gtk::Stack = builder.object("stack").unwrap();
            stack.set_visible_child_name(&format!("{name}page"));
        }));
        let row_to_add =
            if install_vram_management_btn.is_some() { &button_box_t } else { &button_box_s };
        row_to_add.pack_end(&dnsserver_btn, true, true, 2);
    }

    button_box_f.set_halign(gtk::Align::Fill);
    button_box_s.set_halign(gtk::Align::Fill);
    button_box_t.set_halign(gtk::Align::Fill);
    topbox.pack_end(&button_box_t, true, true, 5);
    topbox.pack_end(&button_box_s, true, true, 5);
    topbox.pack_end(&button_box_f, true, true, 5);

    topbox.set_hexpand(true);
    topbox
}

fn create_apps_section() -> Option<gtk::Box> {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let box_collection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("applications"));

    // Check first btn.
    if utils::is_cachyos_pi_installed() {
        let cachyos_pi = gtk::Button::with_label("CachyOS PackageInstaller");
        cachyos_pi.connect_clicked(on_appbtn_clicked);
        box_collection.pack_start(&cachyos_pi, true, true, 2);
    }
    // Check second btn.
    if Path::new("/sbin/cachyos-kernel-manager").exists() {
        let cachyos_km = gtk::Button::with_label("CachyOS Kernel Manager");
        cachyos_km.connect_clicked(on_appbtn_clicked);
        box_collection.pack_start(&cachyos_km, true, true, 2);
    }

    topbox.pack_start(&label, true, true, 5);

    box_collection.set_halign(gtk::Align::Fill);
    topbox.pack_end(&box_collection, true, true, 0);

    topbox.set_hexpand(true);
    if box_collection.children().is_empty() { None } else { Some(topbox) }
}

fn create_troubleshooting_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let button_box_f = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_s = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_t = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("troubleshooting"));

    let removelock_btn = create_gtk_button!("remove-lock-title");
    let reinstall_btn = create_gtk_button!("reinstall-title");
    let resetkeyring_btn = create_gtk_button!("reset-keyrings-title");
    let clear_pkgcache_btn = create_gtk_button!("clear-pkgcache-title");

    let (dialog_tx, dialog_rx) = async_channel::unbounded();
    let dialog_tx_clone = dialog_tx.clone();

    removelock_btn.connect_clicked(move |_| {
        let dialog_tx_clone = dialog_tx_clone.clone();
        std::thread::spawn(move || {
            actions::remove_dblock(dialog_tx_clone);
        });
    });
    reinstall_btn.connect_clicked(move |_| {
        std::thread::spawn(move || {
            actions::reinstall_packages(crate::gui::run_command);
        });
    });
    resetkeyring_btn.connect_clicked(on_resetkeyring_btn_clicked);
    clear_pkgcache_btn.connect_clicked(on_clear_pkgcache_btn_clicked);

    let removelock_btn_clone = removelock_btn.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = dialog_rx.recv().await {
            let widget_obj = match msg.action {
                Action::RemoveLock => &removelock_btn_clone,
                _ => panic!("Unexpected action!!"),
            };
            let widget_window =
                utils::get_window_from_widget(widget_obj).expect("Failed to retrieve window");
            let ui_comp = crate::gui::Gui::new(widget_window);

            ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
        }
    });

    topbox.pack_start(&label, true, false, 1);

    button_box_f.pack_start(&clear_pkgcache_btn, true, true, 2);
    button_box_f.pack_start(&removelock_btn, true, true, 2);
    button_box_f.pack_start(&resetkeyring_btn, true, true, 2);

    button_box_s.pack_start(&reinstall_btn, true, true, 2);

    if utils::is_kwin_wayland() {
        let kwinw_debug_btn = create_gtk_button!("show-kwinw-debug-title");
        kwinw_debug_btn.connect_clicked(move |_| {
            // Spawn child process in separate thread.
            std::thread::spawn(move || {
                // do we even need to start that in separate thread. should be fine without
                actions::launch_kwin_debug_window();
            });
        });
        button_box_t.pack_end(&kwinw_debug_btn, true, true, 2);
    }

    button_box_f.set_halign(gtk::Align::Fill);
    button_box_s.set_halign(gtk::Align::Fill);
    button_box_t.set_halign(gtk::Align::Fill);

    topbox.pack_end(&button_box_t, true, true, 5);
    topbox.pack_end(&button_box_s, true, true, 5);
    topbox.pack_end(&button_box_f, true, true, 5);

    topbox.set_hexpand(true);
    topbox
}

fn create_child_viewport<T: IsA<gtk::Widget>>(child: &T, builder: &Builder, child_name: &str) {
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

    let grid = gtk::Grid::new();
    grid.set_hexpand(true);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_margin_top(5);
    grid.set_margin_bottom(5);
    grid.attach(&back_btn, 0, 1, 1, 1);

    let box_collection_s = gtk::Box::new(gtk::Orientation::Vertical, 5);
    box_collection_s.pack_start(&grid, false, false, 0);
    box_collection_s.pack_start(child, false, false, 10);
    viewport.add(&box_collection_s);

    let stack: gtk::Stack = builder.object("stack").unwrap();
    stack.add_named(&viewport, child_name);
    viewport.show_all();
}

/// Inverse of [`create_child_viewport`]
fn page_content_box(stack: &gtk::Stack, page_name: &str) -> Option<gtk::Box> {
    let viewport = stack.child_by_name(page_name)?.downcast::<gtk::Viewport>().ok()?;
    let box_collection = viewport.child()?.downcast::<gtk::Box>().ok()?;
    box_collection.children().into_iter().nth(1)?.downcast::<gtk::Box>().ok()
}

pub fn create_tweaks_page(builder: &Builder) {
    let install: gtk::Button = builder.object("tweaksBrowser").unwrap();
    install.set_visible(true);
    install.set_label(&fl!("tweaksbrowser-label"));

    // fire cache
    systemd_units::refresh_cache();

    let options_section_box = tweaks::create_options_section();
    let fixes_section_box = create_utilities_section(builder);
    let apps_section_box_opt = create_apps_section();

    let child_name = "tweaksBrowserpage";
    options_section_box.set_widget_name(&format!("{child_name}_options"));
    fixes_section_box.set_widget_name(&format!("{child_name}_fixes"));
    if let Some(apps_section_box) = apps_section_box_opt.as_ref() {
        apps_section_box.set_widget_name(&format!("{child_name}_apps"));
    }

    let box_collection = gtk::Box::new(gtk::Orientation::Vertical, 5);
    box_collection.set_widget_name(child_name);

    box_collection.pack_start(&options_section_box, false, false, 10);
    box_collection.pack_start(&fixes_section_box, false, false, 10);

    if let Some(apps_section_box) = apps_section_box_opt {
        box_collection.pack_end(&apps_section_box, false, false, 10);
    }

    box_collection.set_valign(gtk::Align::Center);
    box_collection.set_halign(gtk::Align::Center);

    create_child_viewport(&box_collection, builder, child_name);
}

pub fn create_appbrowser_page(builder: &Builder) {
    let install: gtk::Button = builder.object("appBrowser").unwrap();
    install.set_visible(utils::is_cachyos_pi_installed());
    install.set_label(&fl!("appbrowser-label"));
    install.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            // Get executable path.
            // TODO(vnepogodin): prompt to install if it doesn't exist
            let exec_path = utils::get_cachyos_pi_path().expect("cachyos-pi not found");
            let exit_status = utils::spawn_detached(&exec_path).expect("Failed to spawn process");
            debug!("Exit status successfully? = {:?}", exit_status.success());
        });
    });
}

pub fn create_troubleshooting_page(builder: &Builder) {
    let troubleshooting_button: gtk::Button = builder.object("troubleshooting").unwrap();
    troubleshooting_button.set_visible(true);
    troubleshooting_button.set_label(&fl!("troubleshooting-label"));

    let child_name = "troubleshootingpage";

    let content: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    content.set_widget_name(child_name);

    let section = create_troubleshooting_section();
    content.pack_start(&section, false, false, 10);

    content.set_valign(gtk::Align::Center);
    content.set_halign(gtk::Align::Center);

    create_child_viewport(&content, builder, child_name);
}

fn on_resetkeyring_btn_clicked(_: &gtk::Button) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        actions::reset_keyring(crate::gui::run_command);
    });
}

fn on_update_system_btn_clicked(_: &gtk::Button) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        actions::update_system(crate::gui::run_command);
    });
}

fn on_clear_pkgcache_btn_clicked(_: &gtk::Button) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        actions::clear_pkgcache(crate::gui::run_command);
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

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        // Get executable path.
        let exec_path = exec_path.unwrap().to_str().unwrap().to_owned();
        let exit_status = utils::spawn_detached(&exec_path).expect("Failed to spawn process");

        debug!("Exit status successfully? = {:?}", exit_status.success());
    });
}
