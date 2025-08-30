use crate::systemd_units::SystemdUnits;
use crate::{actions, fl, systemd_units, utils};

use std::boxed::Box;
use std::path::Path;
use std::str;
use std::sync::Mutex;

use gtk::prelude::*;

use glib::translate::FromGlib;
use gtk::{glib, Builder};
use once_cell::sync::Lazy;
use phf::phf_ordered_map;
use subprocess::Exec;
use tokio::runtime::Runtime;
use tracing::{debug, error};
use which::which;

#[macro_export]
macro_rules! create_gtk_button {
    ($message_id:literal) => {{
        let temp_btn = gtk::Button::with_label(&fl!($message_id));
        temp_btn.set_widget_name($message_id);
        temp_btn
    }};
}

#[macro_export]
macro_rules! create_tweak_checkbox {
    ($tweak_msg:literal,$action_data:literal,$action_type:literal,$alpm_pkg_name:literal) => {{
        let temp_btn =
            gtk::CheckButton::with_label(&fl!("tweak-enabled-title", tweak = $tweak_msg));
        temp_btn.set_widget_name($tweak_msg);

        set_tweak_check_data(&temp_btn, $action_data, $action_type, $alpm_pkg_name);
        connect_tweak(&temp_btn, $action_data);
        temp_btn
    }};
}

static G_LOCAL_UNITS: Lazy<Mutex<SystemdUnits>> = Lazy::new(|| Mutex::new(SystemdUnits::new()));
static G_GLOBAL_UNITS: Lazy<Mutex<SystemdUnits>> = Lazy::new(|| Mutex::new(SystemdUnits::new()));

static G_DNS_SERVERS: phf::OrderedMap<&'static str, (&'static str, &'static str)> = phf_ordered_map! {
    "AdGuard" => ("94.140.14.14,94.140.15.15", "2a10:50c0::ad1:ff,2a10:50c0::ad2:ff"),
    "AdGuard Family Protection" => ("94.140.14.15,94.140.15.16", "2a10:50c0::bad1:ff,2a10:50c0::bad2:ff"),
    "Cloudflare" => ("1.1.1.1,1.0.0.1", "2606:4700:4700::1111,2606:4700:4700::1001"),
    "Cloudflare Malware blocking" => ("1.1.1.2,1.0.0.2", "2606:4700:4700::1112,2606:4700:4700::1002"),
    "Cloudflare Malware and adult content blocking" => ("1.1.1.3,1.0.0.3", "2606:4700:4700::1113,2606:4700:4700::1003"),
    "Cisco Umbrella(OpenDNS)" => ("208.67.222.222,208.67.220.220", "2620:119:35::35,2620:119:53::53"),
    "DNS.Watch" => ("84.200.69.80,84.200.70.40", "2001:1608:10:25::1c04:b12f,2001:1608:10:25::9249:d69b"),
    "GCore" => ("95.85.95.85,2.56.220.2", "2a03:90c0:999d::1,2a03:90c0:9992::1"),
    "Google" => ("8.8.8.8,8.8.4.4", "2001:4860:4860::8888,2001:4860:4860::8844"),
    "Quad9" => ("9.9.9.9,149.112.112.112", "2620:fe::fe,2620:fe::9"),
    "Yandex" => ("77.88.8.8,77.88.8.1", "2a02:6b8::feed:0ff,2a02:6b8:0:1::feed:0ff"),
    "Yandex Malware blocking" => ("77.88.8.88,77.88.8.2", "2a02:6b8::feed:bad,2a02:6b8:0:1::feed:bad"),
    "Yandex Malware and adult content blocking" => ("77.88.8.7,77.88.8.3", "2a02:6b8::feed:a11,2a02:6b8:0:1::feed:a11"),
    "阿里云公共DNS (AliDNS)" => ("223.5.5.5,223.6.6.6", "2400:3200::1,2400:3200:baba::1"),
    "腾讯云 DNSPod (Tencent)" => ("119.29.29.29,119.28.28.28", "2402:4e00::,2402:4e00:1::")
};

pub struct DialogMessage {
    pub msg: String,
    pub msg_type: gtk::MessageType,
    pub action: Action,
}

pub enum Action {
    RemoveLock,
    RemoveOrphans,
    SetDnsServer,
    InstallGaming,
    InstallSnapper,
}

fn update_translation_apps_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(section_label) = section_box_element.clone().downcast::<gtk::Label>() {
            section_label.set_text(&fl!("applications"));
        }
    }
}

fn update_translation_fixes_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(button_box) = section_box_element.clone().downcast::<gtk::Box>() {
            for button_box_widget in button_box.children() {
                let box_element_btn = button_box_widget.downcast::<gtk::Button>().unwrap();
                let widget_name = box_element_btn.widget_name();
                let translated_text = crate::localization::get_locale_text(&widget_name);
                box_element_btn.set_label(&translated_text);
            }
        } else if let Ok(section_label) = section_box_element.downcast::<gtk::Label>() {
            section_label.set_text(&fl!("fixes"));
        }
    }
}

fn update_translation_connections_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(object_box) = section_box_element.clone().downcast::<gtk::Box>() {
            for object_box_widget in object_box.children() {
                let widget_name = object_box_widget.widget_name();
                if let Ok(box_element_btn) = object_box_widget.clone().downcast::<gtk::Button>() {
                    let translated_text = crate::localization::get_locale_text(&widget_name);
                    box_element_btn.set_label(&translated_text);
                } else if let Ok(box_element_label) = object_box_widget.downcast::<gtk::Label>() {
                    let translated_text = crate::localization::get_locale_text(&widget_name);
                    box_element_label.set_text(&translated_text);
                }
            }
        } else if let Ok(section_label) = section_box_element.downcast::<gtk::Label>() {
            section_label.set_text(&fl!("dns-settings"));
        }
    }
}

fn update_translation_options_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(button_box) = section_box_element.clone().downcast::<gtk::Box>() {
            for button_box_widget in button_box.children() {
                let box_element_btn = button_box_widget.downcast::<gtk::Button>().unwrap();
                let widget_name = box_element_btn.widget_name().to_string();
                let translated_text = fl!("tweak-enabled-title", tweak = widget_name);
                box_element_btn.set_label(&translated_text);
            }
        } else if let Ok(section_label) = section_box_element.downcast::<gtk::Label>() {
            section_label.set_text(&fl!("tweaks"));
        }
    }
}

pub fn update_translations(builder: &Builder) {
    // Update buttons
    let tweakbrowser_btn: gtk::Button = builder.object("tweaksBrowser").unwrap();
    tweakbrowser_btn.set_label(&fl!("tweaksbrowser-label"));
    tweakbrowser_btn.set_tooltip_text(Some(&fl!("tweaksbrowser-label")));

    let appbrowser_btn: gtk::Button = builder.object("appBrowser").unwrap();
    appbrowser_btn.set_label(&fl!("appbrowser-label"));
    appbrowser_btn.set_tooltip_text(Some(&fl!("appbrowser-label")));

    let stack: gtk::Stack = builder.object("stack").unwrap();
    {
        if let Some(widget) = stack.child_by_name("tweaksBrowserpage") {
            if let Ok(viewport) = widget.downcast::<gtk::Viewport>() {
                let second_child =
                    &viewport.children()[0].clone().downcast::<gtk::Box>().unwrap().children()[1]
                        .clone()
                        .downcast::<gtk::Box>()
                        .unwrap();

                for second_child_child_widget in second_child.children() {
                    let second_child_child_box =
                        second_child_child_widget.downcast::<gtk::Box>().unwrap();

                    match second_child_child_box.widget_name().as_str() {
                        "tweaksBrowserpage_options" => {
                            update_translation_options_section(&second_child_child_box)
                        },
                        "tweaksBrowserpage_fixes" => {
                            update_translation_fixes_section(&second_child_child_box)
                        },
                        "tweaksBrowserpage_apps" => {
                            update_translation_apps_section(&second_child_child_box)
                        },
                        _ => panic!("Unknown widget!"),
                    }
                }
            }
        }
        if let Some(widget) = stack.child_by_name("dnsConnectionsBrowserpage") {
            if let Ok(viewport) = widget.downcast::<gtk::Viewport>() {
                let second_child =
                    &viewport.children()[0].clone().downcast::<gtk::Box>().unwrap().children()[1]
                        .clone()
                        .downcast::<gtk::Box>()
                        .unwrap();

                for second_child_child_widget in second_child.children() {
                    let second_child_child_box =
                        second_child_child_widget.downcast::<gtk::Box>().unwrap();
                    update_translation_connections_section(&second_child_child_box);
                }
            }
        }
    }
}

fn set_tweak_check_data(
    check_btn: &gtk::CheckButton,
    action_data: &'static str,
    action_type: &'static str,
    alpm_package_name: &'static str,
) {
    unsafe {
        check_btn.set_data("actionData", action_data);
        check_btn.set_data("actionType", action_type);
        check_btn.set_data("alpmPackage", alpm_package_name);
    }
}

fn connect_tweak(check_btn: &gtk::CheckButton, action_data: &'static str) {
    let action_data_str = action_data.to_owned();
    if G_LOCAL_UNITS.lock().unwrap().enabled_units.contains(&action_data_str)
        || G_GLOBAL_UNITS.lock().unwrap().enabled_units.contains(&action_data_str)
    {
        check_btn.set_active(true);
    }
    connect_clicked_and_save(check_btn, on_servbtn_clicked);
}

fn selection_index_for_connection(conn_name: &str) -> usize {
    if let Some((ipv4_dns, ipv6_dns)) = actions::get_dns_for_connection(conn_name) {
        for (key_index, (_name, (ipv4_map, ipv6_map))) in G_DNS_SERVERS.entries().enumerate() {
            if (!ipv4_dns.is_empty() && &ipv4_dns == ipv4_map)
                || (!ipv6_dns.is_empty() && &ipv6_dns == ipv6_map)
            {
                return key_index;
            }
        }
    }

    // fallback to Cloudflare
    G_DNS_SERVERS.get_index("Cloudflare").unwrap()
}

fn create_fixes_section(builder: &Builder) -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let button_box_f = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_s = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_t = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_frth = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("fixes"));

    let removelock_btn = create_gtk_button!("remove-lock-title");
    let reinstall_btn = create_gtk_button!("reinstall-title");
    let resetkeyring_btn = create_gtk_button!("reset-keyrings-title");
    let update_system_btn = create_gtk_button!("update-system-title");
    let remove_orphans_btn = create_gtk_button!("remove-orphans-title");
    let clear_pkgcache_btn = create_gtk_button!("clear-pkgcache-title");
    let rankmirrors_btn = create_gtk_button!("rankmirrors-title");

    let install_gaming_btn = create_gtk_button!("install-gaming-title");
    let install_snapper_btn = create_gtk_button!("install-snapper-title");
    let install_spoof_dpi_btn = create_gtk_button!("install-spoof-dpi-title");

    // Create context channel.
    let (dialog_tx, dialog_rx) = glib::MainContext::channel(glib::Priority::default());

    // Connect signals.
    let dialog_tx_clone = dialog_tx.clone();
    let dialog_tx_gaming = dialog_tx.clone();
    let dialog_tx_snapper = dialog_tx.clone();
    let dialog_tx_spoof = dialog_tx.clone();
    removelock_btn.connect_clicked(move |_| {
        let dialog_tx_clone = dialog_tx_clone.clone();
        std::thread::spawn(move || {
            actions::remove_dblock(dialog_tx_clone);
        });
    });
    reinstall_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            actions::reinstall_packages();
        });
    });
    resetkeyring_btn.connect_clicked(on_resetkeyring_btn_clicked);
    update_system_btn.connect_clicked(on_update_system_btn_clicked);
    remove_orphans_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        let dialog_tx_clone = dialog_tx.clone();
        std::thread::spawn(move || {
            actions::remove_orphans(dialog_tx_clone);
        });
    });
    clear_pkgcache_btn.connect_clicked(on_clear_pkgcache_btn_clicked);
    rankmirrors_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            let _ = utils::run_cmd_terminal(String::from("cachyos-rate-mirrors"), true);
        });
    });
    install_gaming_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        let dialog_tx_gaming = dialog_tx_gaming.clone();
        std::thread::spawn(move || {
            const ALPM_PACKAGE_NAMES: [&str; 2] =
                ["cachyos-gaming-meta", "cachyos-gaming-applications"];
            actions::install_needed_packages(
                &ALPM_PACKAGE_NAMES,
                fl!("gaming-package-installed"),
                Action::InstallGaming,
                dialog_tx_gaming,
            );
        });
    });
    install_snapper_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        let dialog_tx_snapper = dialog_tx_snapper.clone();
        std::thread::spawn(move || {
            actions::install_needed_packages(
                &["cachyos-snapper-support"],
                fl!("snapper-package-installed"),
                Action::InstallSnapper,
                dialog_tx_snapper,
            );
        });
    });
    install_spoof_dpi_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        let dialog_tx_spoof = dialog_tx_spoof.clone();
        std::thread::spawn(move || {
            actions::install_needed_packages(
                &["spoofdpi"],
                fl!("spoof-dpi-package-installed"),
                Action::InstallSnapper,
                dialog_tx_spoof,
            );
        });
    });

    // Setup receiver.
    let removelock_btn_clone = removelock_btn.clone();
    let remove_orphans_btn_clone = remove_orphans_btn.clone();
    let install_gaming_btn_clone = install_gaming_btn.clone();
    let install_snapper_btn_clone = install_snapper_btn.clone();
    dialog_rx.attach(None, move |msg| {
        let widget_obj = match msg.action {
            Action::RemoveLock => &removelock_btn_clone,
            Action::RemoveOrphans => &remove_orphans_btn_clone,
            Action::InstallGaming => &install_gaming_btn_clone,
            Action::InstallSnapper => &install_snapper_btn_clone,
            _ => panic!("Unexpected action!!"),
        };
        let widget_window =
            utils::get_window_from_widget(widget_obj).expect("Failed to retrieve window");

        utils::show_simple_dialog(&widget_window, msg.msg_type, &msg.msg, msg.msg_type.to_string());
        glib::ControlFlow::Continue
    });

    topbox.pack_start(&label, true, false, 1);
    button_box_f.pack_start(&update_system_btn, true, true, 2);
    button_box_f.pack_start(&reinstall_btn, true, true, 2);
    button_box_f.pack_end(&resetkeyring_btn, true, true, 2);
    button_box_s.pack_start(&removelock_btn, true, true, 2);
    button_box_s.pack_start(&clear_pkgcache_btn, true, true, 2);
    button_box_s.pack_end(&remove_orphans_btn, true, true, 2);
    button_box_t.pack_end(&rankmirrors_btn, true, true, 2);
    if utils::is_root_on_btrfs() {
        button_box_t.pack_end(&install_snapper_btn, true, true, 2);
    }
    button_box_t.pack_end(&install_gaming_btn, true, true, 2);
    button_box_frth.pack_end(&install_spoof_dpi_btn, true, true, 2);

    if Path::new("/usr/bin/nmcli").exists() {
        let dnsserver_btn = create_gtk_button!("dnsserver-title");
        dnsserver_btn.connect_clicked(glib::clone!(@weak builder => move |_| {
            let name = "dnsConnectionsBrowser";
            let stack: gtk::Stack = builder.object("stack").unwrap();
            stack.set_visible_child_name(&format!("{name}page"));
        }));
        button_box_frth.pack_end(&dnsserver_btn, true, true, 2);
    }

    button_box_f.set_halign(gtk::Align::Fill);
    button_box_s.set_halign(gtk::Align::Fill);
    button_box_t.set_halign(gtk::Align::Fill);
    button_box_frth.set_halign(gtk::Align::Fill);
    topbox.pack_end(&button_box_frth, true, true, 5);
    topbox.pack_end(&button_box_t, true, true, 5);
    topbox.pack_end(&button_box_s, true, true, 5);
    topbox.pack_end(&button_box_f, true, true, 5);

    if let Ok(pgrep_res) =
        Exec::cmd("pgrep").args(&["kwin_wayland"]).stdout(subprocess::NullFile).join()
    {
        if pgrep_res.success() {
            let kwinw_debug_btn = create_gtk_button!("show-kwinw-debug-title");
            kwinw_debug_btn.connect_clicked(move |_| {
                // Spawn child process in separate thread.
                std::thread::spawn(move || {
                    // do we even need to start that in separate thread. should be fine without
                    actions::launch_kwin_debug_window();
                });
            });
            button_box_frth.pack_end(&kwinw_debug_btn, true, true, 2);
        }
    }

    topbox.set_hexpand(true);
    topbox
}

fn create_options_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let box_collection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let box_collection_s = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("tweaks"));

    let psd_btn = create_tweak_checkbox!(
        "Profile-sync-daemon",
        "psd.service",
        "user_service",
        "profile-sync-daemon"
    );
    let systemd_oomd_btn =
        create_tweak_checkbox!("Systemd-oomd", "systemd-oomd.service", "service", "");
    let bpftune_btn =
        create_tweak_checkbox!("Bpftune", "bpftune.service", "service", "bpftune-git");
    let bluetooth_btn =
        create_tweak_checkbox!("Bluetooth", "bluetooth.service", "service", "bluez");
    let ananicy_cpp_btn =
        create_tweak_checkbox!("Ananicy Cpp", "ananicy-cpp.service", "service", "ananicy-cpp");
    let cachy_update_btn =
        create_tweak_checkbox!("Cachy Update", "arch-update.timer arch-update-tray.service", "user_service", "cachy-update");

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

fn create_apps_section() -> Option<gtk::Box> {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let box_collection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("applications"));

    // Check first btn.
    if Path::new("/sbin/cachyos-pi").exists() {
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
    match !box_collection.children().is_empty() {
        true => Some(topbox),
        _ => None,
    }
}

fn create_connections_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let connection_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let dnsservers_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text(&fl!("dns-settings"));

    let connections_label = gtk::Label::new(None);
    connections_label.set_justify(gtk::Justification::Left);
    connections_label.set_text(&fl!("select-connection"));
    connections_label.set_widget_name("select-connection");
    let servers_label = gtk::Label::new(None);
    servers_label.set_justify(gtk::Justification::Left);
    servers_label.set_text(&fl!("select-dns-server"));
    servers_label.set_widget_name("select-dns-server");
    let apply_btn = create_gtk_button!("apply");
    let reset_btn = create_gtk_button!("reset");

    let combo_conn = {
        let store = gtk::ListStore::new(&[String::static_type()]);
        let nm_connections = actions::get_nm_connections();
        for nm_connection in nm_connections.iter() {
            store.set(&store.append(), &[(0, nm_connection)]);
        }
        utils::create_combo_with_model(&store)
    };
    let combo_servers = {
        let store = gtk::ListStore::new(&[String::static_type()]);
        for dns_server in G_DNS_SERVERS.keys() {
            store.set(&store.append(), &[(0, dns_server)]);
        }
        utils::create_combo_with_model(&store)
    };

    combo_conn.set_widget_name("connections_combo");
    combo_servers.set_widget_name("servers_combo");

    // preset the current active connection
    if let Some(active_conn_name) = actions::get_active_connection_name() {
        let model = combo_conn.model().unwrap();
        if let Some(iter) = model.iter_first() {
            loop {
                if model.value(&iter, 0).get::<String>().unwrap() == active_conn_name {
                    combo_conn.set_active_iter(Some(&iter));

                    let selected_dns_index = selection_index_for_connection(&active_conn_name);
                    combo_servers.set_active(Some(selected_dns_index as u32));
                    break;
                }
                if !model.iter_next(&iter) {
                    break;
                }
            }
        }
    }

    // select used dns option value on connection change
    let combo_servers_clone = combo_servers.clone();
    combo_conn.connect_changed(move |combo| {
        let conn_name = if let Some(tree_iter) = combo.active_iter() {
            let model = combo.model().unwrap();
            model.value(&tree_iter, 0).get::<String>().unwrap()
        } else {
            // use empty string which will trigger fallback
            "".to_owned()
        };

        let selected_dns_index = selection_index_for_connection(&conn_name);
        combo_servers_clone.set_active(Some(selected_dns_index as u32));
    });

    // Create context channel.
    let (dialog_tx, dialog_rx) = glib::MainContext::channel(glib::Priority::default());

    // Connect signals.
    let dialog_tx_clone = dialog_tx.clone();
    let combo_conn_clone = combo_conn.clone();
    let combo_serv_clone = combo_servers.clone();
    apply_btn.connect_clicked(move |_| {
        let dialog_tx_clone = dialog_tx_clone.clone();
        let conn_name = {
            if let Some(tree_iter) = combo_conn_clone.active_iter() {
                let model = combo_conn_clone.model().unwrap();
                let group_gobj = model.value(&tree_iter, 0);
                let group = group_gobj.get::<&str>().unwrap();
                String::from(group)
            } else {
                "".into()
            }
        };
        let server_name = {
            if let Some(tree_iter) = combo_serv_clone.active_iter() {
                let model = combo_serv_clone.model().unwrap();
                let group_gobj = model.value(&tree_iter, 0);
                let group = group_gobj.get::<&str>().unwrap();
                String::from(group)
            } else {
                "".into()
            }
        };
        let server_addr = G_DNS_SERVERS.get(&server_name).unwrap();
        std::thread::spawn(move || {
            actions::change_dns_server(&conn_name, server_addr.0, server_addr.1, dialog_tx_clone);
        });
    });
    let dialog_tx_clone = dialog_tx.clone();
    let combo_conn_clone = combo_conn.clone();
    reset_btn.connect_clicked(move |_| {
        let dialog_tx_clone = dialog_tx_clone.clone();
        let conn_name = {
            if let Some(tree_iter) = combo_conn_clone.active_iter() {
                let model = combo_conn_clone.model().unwrap();
                let group_gobj = model.value(&tree_iter, 0);
                let group = group_gobj.get::<&str>().unwrap();
                String::from(group)
            } else {
                "".into()
            }
        };
        std::thread::spawn(move || {
            actions::reset_dns_server(&conn_name, dialog_tx_clone);
        });
    });

    // Setup receiver
    let apply_btn_clone = apply_btn.clone();
    dialog_rx.attach(None, move |msg| {
        let widget_obj = &apply_btn_clone;
        let widget_window =
            utils::get_window_from_widget(widget_obj).expect("Failed to retrieve window");

        utils::show_simple_dialog(&widget_window, msg.msg_type, &msg.msg, msg.msg_type.to_string());
        glib::ControlFlow::Continue
    });

    topbox.pack_start(&label, true, false, 1);
    connection_box.pack_start(&connections_label, true, true, 2);
    connection_box.pack_end(&combo_conn, true, true, 2);
    dnsservers_box.pack_start(&servers_label, true, true, 2);
    dnsservers_box.pack_end(&combo_servers, true, true, 2);
    button_box.pack_start(&reset_btn, true, true, 2);
    button_box.pack_end(&apply_btn, true, true, 2);
    connection_box.set_halign(gtk::Align::Fill);
    dnsservers_box.set_halign(gtk::Align::Fill);
    button_box.set_halign(gtk::Align::Fill);
    topbox.pack_start(&connection_box, true, true, 5);
    topbox.pack_start(&dnsservers_box, true, true, 5);
    topbox.pack_start(&button_box, true, true, 5);

    topbox.set_hexpand(true);
    topbox
}

fn load_enabled_units() {
    G_LOCAL_UNITS.lock().unwrap().enabled_units.clear();

    let rt = Runtime::new().expect("Failed to initialize tokio runtime");
    let res = rt.block_on(async move {
        let units = systemd_units::get_enabled_global_units().await?;
        G_LOCAL_UNITS.lock().unwrap().enabled_units = units;

        anyhow::Ok(())
    });

    if let Err(res_err) = res {
        error!("Failed to load systemd units: {res_err}");
    }
}

fn load_global_enabled_units() {
    G_GLOBAL_UNITS.lock().unwrap().enabled_units.clear();

    let rt = Runtime::new().expect("Failed to initialize tokio runtime");
    let res = rt.block_on(async move {
        let units = systemd_units::get_enabled_user_units().await?;
        G_GLOBAL_UNITS.lock().unwrap().enabled_units = units;

        anyhow::Ok(())
    });

    if let Err(res_err) = res {
        error!("Failed to load user systemd units: {res_err}");
    }
}

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
    install.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            // Get executable path.
            let exec_path = "/usr/bin/cachyos-pi";
            let exit_status = Exec::cmd(exec_path).detached().join().expect("Failed to spawn process");

            debug!("Exit status successfully? = {:?}", exit_status.success());
        });
    });
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

    let action_enabled = action_data.split(' ').all(|x| units_handle.enabled_units.contains(&x.to_owned()));
    let cmd = if !action_enabled {
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

fn on_resetkeyring_btn_clicked(_: &gtk::Button) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        actions::reset_keyring();
    });
}

fn on_update_system_btn_clicked(_: &gtk::Button) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        actions::update_system();
    });
}

fn on_clear_pkgcache_btn_clicked(_: &gtk::Button) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        actions::clear_pkgcache();
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
        let exit_status = Exec::cmd(exec_path).detached().join().expect("Failed to spawn process");

        debug!("Exit status successfully? = {:?}", exit_status.success());
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
