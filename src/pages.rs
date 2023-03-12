use crate::application_browser::ApplicationBrowser;
use crate::data_types::*;
use crate::utils;
use crate::utils::PacmanWrapper;
use gtk::{glib, Builder};
use once_cell::sync::Lazy;
use std::fmt::Write as _;
use std::path::Path;
use std::sync::Mutex;

use gtk::prelude::*;

use std::str;
use subprocess::{Exec, Redirection};

static G_LOCAL_UNITS: Lazy<Mutex<SystemdUnits>> = Lazy::new(|| Mutex::new(SystemdUnits::new()));
static G_GLOBAL_UNITS: Lazy<Mutex<SystemdUnits>> = Lazy::new(|| Mutex::new(SystemdUnits::new()));

fn create_fixes_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let button_box_f = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_s = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box_t = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text("Fixes");

    let removelock_btn = gtk::Button::with_label("Remove db lock");
    let reinstall_btn = gtk::Button::with_label("Reinstall all packages");
    let refreshkeyring_btn = gtk::Button::with_label("Refresh keyrings");
    let update_system_btn = gtk::Button::with_label("System update");
    let remove_orphans_btn = gtk::Button::with_label("Remove orphans");
    let clear_pkgcache_btn = gtk::Button::with_label("Clear package cache");
    let rankmirrors_btn = gtk::Button::with_label("Rank mirrors");

    removelock_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            if Path::new("/var/lib/pacman/db.lck").exists() {
                let _ = Exec::cmd("/sbin/pkexec")
                    .arg("bash")
                    .arg("-c")
                    .arg("rm /var/lib/pacman/db.lck")
                    .join()
                    .unwrap();
                if !Path::new("/var/lib/pacman/db.lck").exists() {
                    let dialog = gtk::MessageDialog::builder()
                        .message_type(gtk::MessageType::Info)
                        .text("Pacman db lock was removed!")
                        .build();
                    dialog.show();
                }
            }
        });
    });
    reinstall_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            let _ = utils::run_cmd_terminal(String::from("pacman -S $(pacman -Qnq)"), true);
        });
    });
    refreshkeyring_btn.connect_clicked(on_refreshkeyring_btn_clicked);
    update_system_btn.connect_clicked(on_update_system_btn_clicked);
    remove_orphans_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
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
                let dialog = gtk::MessageDialog::builder()
                    .message_type(gtk::MessageType::Info)
                    .text("No orphan packages found!")
                    .build();
                dialog.show();
                return;
            }
            let _ = utils::run_cmd_terminal(format!("pacman -Rns {orphan_pkgs}"), true);
        });
    });
    clear_pkgcache_btn.connect_clicked(on_clear_pkgcache_btn_clicked);
    rankmirrors_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            let _ = utils::run_cmd_terminal(String::from("cachyos-rate-mirrors"), true);
        });
    });

    topbox.pack_start(&label, true, false, 1);
    button_box_f.pack_start(&update_system_btn, true, true, 2);
    button_box_f.pack_start(&reinstall_btn, true, true, 2);
    button_box_f.pack_end(&refreshkeyring_btn, true, true, 2);
    button_box_s.pack_start(&removelock_btn, true, true, 2);
    button_box_s.pack_start(&clear_pkgcache_btn, true, true, 2);
    button_box_s.pack_end(&remove_orphans_btn, true, true, 2);
    button_box_t.pack_end(&rankmirrors_btn, true, true, 2);
    button_box_f.set_halign(gtk::Align::Fill);
    button_box_s.set_halign(gtk::Align::Fill);
    button_box_t.set_halign(gtk::Align::Fill);
    topbox.pack_end(&button_box_t, true, true, 5);
    topbox.pack_end(&button_box_s, true, true, 5);
    topbox.pack_end(&button_box_f, true, true, 5);

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
    label.set_text("Tweaks");

    let psd_btn = gtk::CheckButton::with_label("Profile-sync-daemon enable");
    let systemd_oomd_btn = gtk::CheckButton::with_label("Systemd-oomd enabled");
    let apparmor_btn = gtk::CheckButton::with_label("Apparmor enabled");
    let bluetooth_btn = gtk::CheckButton::with_label("Bluetooth enabled");
    let ananicy_cpp_btn = gtk::CheckButton::with_label("Ananicy Cpp enabled");
    let dnscrypt_btn = gtk::CheckButton::with_label("DNSCrypt enabled");

    unsafe {
        psd_btn.set_data("actionData", "psd.service");
        psd_btn.set_data("actionType", "user_service");
        systemd_oomd_btn.set_data("actionData", "systemd-oomd.service");
        systemd_oomd_btn.set_data("actionType", "service");
        apparmor_btn.set_data("actionData", "apparmor.service");
        apparmor_btn.set_data("actionType", "service");
        bluetooth_btn.set_data("actionData", "bluetooth.service");
        bluetooth_btn.set_data("actionType", "service");
        ananicy_cpp_btn.set_data("actionData", "ananicy-cpp.service");
        ananicy_cpp_btn.set_data("actionType", "service");
    }

    for btn in &[&psd_btn, &systemd_oomd_btn, &apparmor_btn, &bluetooth_btn, &ananicy_cpp_btn] {
        unsafe {
            let data: &str = *btn.data("actionData").unwrap().as_ptr();
            if G_LOCAL_UNITS.lock().unwrap().enabled_units.contains(&String::from(data))
                || G_GLOBAL_UNITS.lock().unwrap().enabled_units.contains(&String::from(data))
            {
                btn.set_active(true);
            }
        }
    }

    let check_is_pkg_installed = |pkg_name: &str| -> bool {
        let pacman =
            pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
        let alpm = alpm_utils::alpm_with_conf(&pacman).unwrap();

        matches!(alpm.localdb().pkg(pkg_name.as_bytes()), Ok(_))
    };
    let is_local_service_enabled = |service_unit_name: &str| -> bool {
        let local_units = &G_LOCAL_UNITS.lock().unwrap().enabled_units;
        local_units.contains(&String::from(service_unit_name))
    };

    {
        let pkg_name = "cachyos-dnscrypt-proxy";
        let service_unit_name = "dnscrypt-proxy.service";

        let is_installed = check_is_pkg_installed(pkg_name);
        let service_enabled = is_local_service_enabled(service_unit_name);
        let is_valid = is_installed && service_enabled;
        if is_valid {
            dnscrypt_btn.set_active(true);
        }
    };

    psd_btn.connect_clicked(on_servbtn_clicked);
    systemd_oomd_btn.connect_clicked(on_servbtn_clicked);
    apparmor_btn.connect_clicked(on_servbtn_clicked);
    bluetooth_btn.connect_clicked(on_servbtn_clicked);
    ananicy_cpp_btn.connect_clicked(on_servbtn_clicked);
    dnscrypt_btn.connect_clicked(move |_| {
        // Spawn child process in separate thread.
        std::thread::spawn(move || {
            let pkg_name = "cachyos-dnscrypt-proxy";
            let service_unit_name = "dnscrypt-proxy.service";
            if !check_is_pkg_installed(pkg_name) {
                let _ = utils::run_cmd_terminal(format!("pacman -S {pkg_name}"), true);
                load_enabled_units();
                return;
            }
            if !is_local_service_enabled(service_unit_name) {
                load_enabled_units();
            }
        });
    });

    topbox.pack_start(&label, true, false, 1);
    box_collection.pack_start(&psd_btn, true, false, 2);
    box_collection_s.pack_start(&systemd_oomd_btn, true, false, 2);
    box_collection.pack_start(&apparmor_btn, true, false, 2);
    box_collection.pack_start(&ananicy_cpp_btn, true, false, 2);
    box_collection_s.pack_start(&dnscrypt_btn, true, false, 2);
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
    label.set_text("Applications");

    // Check first btn.
    if Path::new("/sbin/cachyos-pi-bin").exists() {
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

fn load_enabled_units() {
    G_LOCAL_UNITS.lock().unwrap().loaded_units.clear();
    G_LOCAL_UNITS.lock().unwrap().enabled_units.clear();

    let mut exec_out = Exec::shell("systemctl list-unit-files -q --no-pager | tr -s \" \"")
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();
    exec_out.pop();

    let service_list = exec_out.split('\n');

    for service in service_list {
        let out: Vec<&str> = service.split(' ').collect();
        G_LOCAL_UNITS.lock().unwrap().loaded_units.push(String::from(out[0]));
        if out[1] == "enabled" {
            G_LOCAL_UNITS.lock().unwrap().enabled_units.push(String::from(out[0]));
        }
    }
}

fn load_global_enabled_units() {
    G_GLOBAL_UNITS.lock().unwrap().loaded_units.clear();
    G_GLOBAL_UNITS.lock().unwrap().enabled_units.clear();

    let mut exec_out =
        Exec::shell("systemctl --global list-unit-files -q --no-pager | tr -s \" \"")
            .stdout(Redirection::Pipe)
            .capture()
            .unwrap()
            .stdout_str();
    exec_out.pop();

    let service_list = exec_out.split('\n');
    for service in service_list {
        let out: Vec<&str> = service.split(' ').collect();
        G_GLOBAL_UNITS.lock().unwrap().loaded_units.push(String::from(out[0]));
        if out[1] == "enabled" {
            G_GLOBAL_UNITS.lock().unwrap().enabled_units.push(String::from(out[0]));
        }
    }
}

pub fn create_tweaks_page(builder: &Builder) {
    let install: gtk::Button = builder.object("tweaksBrowser").unwrap();
    install.set_visible(true);

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
    let fixes_section_box = create_fixes_section();
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

pub fn create_appbrowser_page(builder: &Builder) {
    let install: gtk::Button = builder.object("appBrowser").unwrap();
    install.set_visible(true);

    let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    let app_browser_ref = ApplicationBrowser::default_impl().lock().unwrap();
    app_browser_ref.back_btn.connect_clicked(glib::clone!(@weak builder => move |button| {
        let name = button.widget_name();
        let stack: gtk::Stack = builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{name}page"));
    }));
    let app_browser_box = app_browser_ref.get_page();

    // Add grid to the viewport
    // NOTE: we might eliminate that?
    viewport.add(app_browser_box);
    viewport.show_all();

    let stack: gtk::Stack = builder.object("stack").unwrap();
    let child_name = "appBrowserpage";
    stack.add_named(&viewport, child_name);
}

fn on_servbtn_clicked(button: &gtk::CheckButton) {
    // Get action data/type.
    let action_type: &str;
    let action_data: &str;
    unsafe {
        action_type = *button.data("actionType").unwrap().as_ptr();
        action_data = *button.data("actionData").unwrap().as_ptr();
    }

    let (user_only, pkexec_only) =
        if action_type == "user_service" { ("--user", "--user $(logname)") } else { ("", "") };

    let local_units = &G_LOCAL_UNITS.lock().unwrap().enabled_units;
    let cmd = if !local_units.contains(&String::from(action_data)) {
        format!(
            "/sbin/pkexec {pkexec_only} bash -c \"systemctl {user_only} enable --now --force \
             {action_data}\""
        )
    } else {
        format!(
            "/sbin/pkexec {pkexec_only} bash -c \"systemctl {user_only} disable --now \
             {action_data}\""
        )
    };

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        Exec::shell(cmd).join().unwrap();

        if action_type == "user_service" {
            load_global_enabled_units();
        } else {
            load_enabled_units();
        }
    });
}

fn on_refreshkeyring_btn_clicked(_: &gtk::Button) {
    let pacman = pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
    let alpm = alpm_utils::alpm_with_conf(&pacman).unwrap();
    // pacman -Qq | grep keyring
    let needles = alpm
        .localdb()
        .search([".*-keyring"].iter())
        .unwrap()
        .into_iter()
        .filter(|pkg| pkg.name() != "gnome-keyring")
        .map(|pkg| {
            let mut pkgname = String::from(pkg.name());
            pkgname.remove_matches("-keyring");
            format!("{pkgname} ")
        })
        .collect::<String>();

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let _ = utils::run_cmd_terminal(
            format!("pacman-key --init && pacman-key --populate {needles}"),
            true,
        );
    });
}

fn on_update_system_btn_clicked(_: &gtk::Button) {
    let (cmd, escalate) = match utils::get_pacman_wrapper() {
        PacmanWrapper::Pak => ("pak -Syu", false),
        PacmanWrapper::Yay => ("yay -Syu", false),
        PacmanWrapper::Paru => ("paru --removemake -Syu", false),
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
    let (binname, is_sudo) = if name == "CachyOS PackageInstaller" {
        ("cachyos-pi-bin", true)
    } else if name == "CachyOS Kernel Manager" {
        ("cachyos-kernel-manager", false)
    } else {
        ("", false)
    };

    // Check if executable exists.
    let exit_status = Exec::cmd("which").arg(binname).join().unwrap();
    if !exit_status.success() {
        return;
    }

    let mut envs = String::new();
    for env in glib::listenv() {
        if env == "PATH" {
            envs += "PATH=/sbin:/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/sbin ";
            continue;
        }
        let _ = write!(
            envs,
            "{}=\"{}\" ",
            env.to_str().unwrap(),
            glib::getenv(&env).unwrap().to_str().unwrap()
        );
    }

    // Get executable path.
    let mut exe_path =
        Exec::cmd("which").arg(binname).stdout(Redirection::Pipe).capture().unwrap().stdout_str();
    exe_path.pop();
    let bash_cmd = format!("{} {}", &envs, &exe_path);

    // Create context channel.
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let exit_status = if is_sudo {
            Exec::cmd("/sbin/pkexec").arg("bash").arg("-c").arg(bash_cmd).join().unwrap()
        } else {
            Exec::shell(bash_cmd).join().unwrap()
        };
        tx.send(format!("Exit status successfully? = {:?}", exit_status.success()))
            .expect("Couldn't send data to channel");
    });

    rx.attach(None, move |text| {
        println!("{text}");
        glib::Continue(true)
    });
}
