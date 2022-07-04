// extern crate gtk;

use crate::application_browser::ApplicationBrowser;
use crate::data_types::*;
use gtk::{glib, Builder};
use once_cell::sync::Lazy;
use std::fmt::Write as _;
use std::sync::Mutex;

use gtk::prelude::*;

use std::str;
use subprocess::{Exec, Redirection};

static mut g_local_units: Lazy<Mutex<SystemdUnits>> = Lazy::new(|| Mutex::new(SystemdUnits::new()));
static mut g_global_units: Lazy<Mutex<SystemdUnits>> =
    Lazy::new(|| Mutex::new(SystemdUnits::new()));

fn create_options_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let box_collection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text("Tweaks");

    let psd_btn = gtk::CheckButton::with_label("Profile-sync-daemon enable");
    let systemd_oomd_btn = gtk::CheckButton::with_label("Systemd-oomd enabled");
    let apparmor_btn = gtk::CheckButton::with_label("Apparmor enabled");
    let ananicy_cpp_btn = gtk::CheckButton::with_label("Ananicy Cpp enabled");

    unsafe {
        psd_btn.set_data("actionData", "psd.service");
        psd_btn.set_data("actionType", "user_service");
        systemd_oomd_btn.set_data("actionData", "systemd-oomd.service");
        systemd_oomd_btn.set_data("actionType", "service");
        apparmor_btn.set_data("actionData", "apparmor.service");
        apparmor_btn.set_data("actionType", "service");
        ananicy_cpp_btn.set_data("actionData", "ananicy-cpp.service");
        ananicy_cpp_btn.set_data("actionType", "service");
    }

    for btn in &[&psd_btn, &systemd_oomd_btn, &apparmor_btn, &ananicy_cpp_btn] {
        unsafe {
            let data: &str = *btn.data("actionData").unwrap().as_ptr();
            if g_local_units.lock().unwrap().enabled_units.contains(&String::from(data))
                || g_global_units.lock().unwrap().enabled_units.contains(&String::from(data))
            {
                btn.set_active(true);
            }
        }
    }

    psd_btn.connect_clicked(on_servbtn_clicked);
    systemd_oomd_btn.connect_clicked(on_servbtn_clicked);
    apparmor_btn.connect_clicked(on_servbtn_clicked);
    ananicy_cpp_btn.connect_clicked(on_servbtn_clicked);

    topbox.pack_start(&label, true, false, 1);
    box_collection.pack_start(&psd_btn, true, false, 2);
    box_collection.pack_start(&systemd_oomd_btn, true, false, 2);
    box_collection.pack_start(&apparmor_btn, true, false, 2);
    box_collection.pack_start(&ananicy_cpp_btn, true, false, 2);
    box_collection.set_halign(gtk::Align::Fill);
    topbox.pack_start(&box_collection, true, false, 1);

    topbox.set_hexpand(true);
    topbox
}

fn create_apps_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let box_collection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(None);
    label.set_line_wrap(true);
    label.set_justify(gtk::Justification::Center);
    label.set_text("Applications");

    let cachyos_pi = gtk::Button::with_label("CachyOS PackageInstaller");
    let cachyos_km = gtk::Button::with_label("CachyOS Kernel Manager");

    cachyos_pi.connect_clicked(on_appbtn_clicked);
    cachyos_km.connect_clicked(on_appbtn_clicked);

    box_collection.pack_start(&cachyos_pi, true, true, 2);
    box_collection.pack_start(&cachyos_km, true, true, 2);

    topbox.pack_start(&label, true, true, 2);

    box_collection.set_halign(gtk::Align::Fill);
    topbox.pack_start(&box_collection, true, true, 0);

    topbox.set_hexpand(true);
    topbox
}

fn load_enabled_units() {
    unsafe {
        g_local_units.lock().unwrap().loaded_units.clear();
        g_local_units.lock().unwrap().enabled_units.clear();

        let mut exec_out = Exec::shell("systemctl list-unit-files -q --no-pager | tr -s \" \"")
            .stdout(Redirection::Pipe)
            .capture()
            .unwrap()
            .stdout_str();
        exec_out.pop();

        let service_list = exec_out.split('\n');

        for service in service_list {
            let out: Vec<&str> = service.split(' ').collect();
            g_local_units.lock().unwrap().loaded_units.push(out[0].to_string());
            if out[1] == "enabled" {
                g_local_units.lock().unwrap().enabled_units.push(out[0].to_string());
            }
        }
    }
}

fn load_global_enabled_units() {
    unsafe {
        g_global_units.lock().unwrap().loaded_units.clear();
        g_global_units.lock().unwrap().enabled_units.clear();

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
            g_global_units.lock().unwrap().loaded_units.push(out[0].to_string());
            if out[1] == "enabled" {
                g_global_units.lock().unwrap().enabled_units.push(out[0].to_string());
            }
        }
    }
}

pub fn create_tweaks_page(builder: &Builder) {
    let install: gtk::Button = builder.object("tweaksBrowser").unwrap();
    install.set_visible(true);

    load_enabled_units();
    load_global_enabled_units();

    let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    // let label = gtk::Label::new(None);
    // label.set_line_wrap(true);
    let image = gtk::Image::from_icon_name(Some("go-previous"), gtk::IconSize::Button);
    let back_btn = gtk::Button::new();
    back_btn.set_image(Some(&image));
    back_btn.set_widget_name("home");

    back_btn.connect_clicked(glib::clone!(@weak builder => move |button| {
        let name = button.widget_name();
        let stack: gtk::Stack = builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{}page", name));
    }));

    let options_section_box = create_options_section();
    let apps_section_box = create_apps_section();

    let grid = gtk::Grid::new();
    grid.set_hexpand(true);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_margin_top(5);
    grid.set_margin_bottom(5);
    grid.attach(&back_btn, 0, 1, 1, 1);
    let box_collection = gtk::Box::new(gtk::Orientation::Vertical, 5);

    box_collection.pack_start(&options_section_box, true, true, 5);
    box_collection.pack_start(&apps_section_box, true, true, 5);

    box_collection.set_valign(gtk::Align::Center);
    box_collection.set_halign(gtk::Align::Center);
    grid.attach(&box_collection, 1, 2, 5, 1);
    viewport.add(&grid);
    viewport.show_all();

    let stack: gtk::Stack = builder.object("stack").unwrap();
    let child_name = "tweaksBrowserpage";
    stack.add_named(&viewport, child_name);
}

pub fn create_appbrowser_page(builder: &Builder) {
    let install: gtk::Button = builder.object("appBrowser").unwrap();
    install.set_visible(true);

    let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    // let label = gtk::Label::new(None);
    // label.set_line_wrap(true);
    let image = gtk::Image::from_icon_name(Some("go-previous"), gtk::IconSize::Button);
    let back_btn = gtk::Button::new();
    back_btn.set_image(Some(&image));
    back_btn.set_widget_name("home");

    back_btn.connect_clicked(glib::clone!(@weak builder => move |button| {
        let name = button.widget_name();
        let stack: gtk::Stack = builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{}page", name));
    }));

    let grid = gtk::Grid::new();
    grid.set_hexpand(true);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_margin_top(5);
    grid.set_margin_bottom(5);
    grid.attach(&back_btn, 0, 1, 1, 1);

    let app_browser_ref = ApplicationBrowser::default_impl().lock().unwrap();
    let app_browser_box = app_browser_ref.get_page();
    grid.attach(app_browser_box, 0, 2, 1, 1);

    // Add grid to the viewport
    // NOTE: we might eliminate that?
    viewport.add(&grid);
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

    let cmd: String;
    unsafe {
        let local_units = &g_local_units.lock().unwrap().enabled_units;
        cmd = if !local_units.contains(&String::from(action_data)) {
            format!(
                "/sbin/pkexec {} bash -c \"systemctl {} enable --now --force {}\"",
                pkexec_only, user_only, action_data
            )
        } else {
            format!(
                "/sbin/pkexec {} bash -c \"systemctl {} disable --now {}\"",
                pkexec_only, user_only, action_data
            )
        };
    }

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
        println!("{}", text);
        glib::Continue(true)
    });
}
