#![feature(const_str_from_utf8)]
#![feature(string_remove_matches)]
#![allow(non_upper_case_globals)]

mod alpm_helper;
mod application_browser;
mod config;
mod data_types;
mod pages;
mod utils;

use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR, PROFILE, VERSION};
use data_types::*;
use gettextrs::LocaleCategory;
use gtk::{gio, glib, Builder, HeaderBar, Window};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use utils::*;

use gio::prelude::*;
use gtk::prelude::*;

use gdk_pixbuf::Pixbuf;

use serde_json::json;
use std::{fs, str};
use subprocess::Exec;

static mut g_save_json: Lazy<Mutex<serde_json::Value>> = Lazy::new(|| Mutex::new(json!(null)));

static mut g_hello_window: Option<Arc<HelloWindow>> = None;

fn quick_message(message: &'static str) {
    // Create the widgets
    let dialog = gtk::Dialog::builder().title(message).modal(true).build();

    dialog.set_destroy_with_parent(true);
    dialog.add_button("_Offline", gtk::ResponseType::No);
    dialog.add_button("_Online", gtk::ResponseType::Yes);
    let content_area = dialog.content_area();
    let label = gtk::Label::new(Some(message));

    // Add the label, and show everything we’ve added
    content_area.add(&label);
    dialog.show_all();

    let result = dialog.run();
    let cmd: String;
    if result == gtk::ResponseType::No {
        cmd = fix_path("/usr/local/bin/calamares-offline.sh");
    } else if result == gtk::ResponseType::Yes {
        cmd = fix_path("/usr/local/bin/calamares-online.sh");
    } else {
        unsafe {
            dialog.destroy();
        }
        return;
    }

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let status = match reqwest::blocking::get("https://cachyos.org") {
            Ok(resp) => resp.status().is_success() || resp.status().is_server_error(),
            _ => false,
        };

        if !status && result == gtk::ResponseType::Yes {
            let errordialog = gtk::MessageDialog::builder()
                .title(message)
                .text("Unable to start online installation! No internet connection")
                .modal(true)
                .message_type(gtk::MessageType::Error)
                .build();
            errordialog.show();
            return;
        }

        Exec::shell(cmd).join().unwrap();
    });

    unsafe {
        dialog.destroy();
    }
}

fn show_about_dialog() {
    let main_window: Window;
    unsafe {
        main_window = g_hello_window.clone().unwrap().window.clone();
    }
    let logo_path = format!("/usr/share/icons/hicolor/scalable/apps/{}.svg", APP_ID);
    let logo = Pixbuf::from_file(logo_path).unwrap();

    let dialog = gtk::AboutDialog::builder()
        .transient_for(&main_window)
        .modal(true)
        .program_name(&gettextrs::gettext("CachyOS Hello"))
        .comments(&gettextrs::gettext("Welcome screen for CachyOS"))
        .version(VERSION)
        .logo(&logo)
        .authors(vec![
            "Vladislav Nepogodin".into(),
        ])
        // Translators: Replace "translator-credits" with your names. Put a comma between.
        .translator_credits(&gettextrs::gettext("translator-credits"))
        .copyright("2021-2023 CachyOS team")
        .license_type(gtk::License::Gpl30)
        .website("https://github.com/cachyos/cachyos-welcome")
        .website_label("GitHub")
        .build();

    dialog.run();
    dialog.hide();
}

fn main() {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain.");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain.");

    glib::set_application_name("CachyOSHello");

    gtk::init().expect("Unable to start GTK3.");

    let application = gtk::Application::new(
        Some(APP_ID),       // Application id
        Default::default(), // Using default flags
    );

    application.connect_activate(|application| {
        build_ui(application);
    });

    // Run the application and start the event loop
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let data = fs::read_to_string(format!("{}/data/preferences.json", PKGDATADIR))
        .expect("Unable to read file");
    let preferences: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

    // Get saved infos
    let save_path = fix_path(preferences["save_path"].as_str().unwrap());
    let save: serde_json::Value = if !Path::new(&save_path).exists() {
        json!({"locale": ""})
    } else {
        read_json(save_path.as_str())
    };

    // Import Css
    let provider = gtk::CssProvider::new();
    provider
        .load_from_path(preferences["style_path"].as_str().unwrap())
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Init window
    let builder: Builder = Builder::from_file(preferences["ui_path"].as_str().unwrap());
    builder.connect_signals(|_builder, handler_name| {
        match handler_name {
            // handler_name as defined in the glade file => handler function as defined above
            "on_languages_changed" => Box::new(on_languages_changed),
            "on_action_clicked" => Box::new(on_action_clicked),
            "on_btn_clicked" => Box::new(on_btn_clicked),
            "on_link_clicked" => Box::new(on_link_clicked),
            "on_link1_clicked" => Box::new(on_link1_clicked),
            "on_delete_window" => Box::new(on_delete_window),
            _ => Box::new(|_| None),
        }
    });

    let main_window: Window = builder.object("window").expect("Could not get the object window");
    main_window.set_application(Some(application));

    unsafe {
        g_hello_window = Some(Arc::new(HelloWindow {
            window: main_window.clone(),
            builder: builder.clone(),
            preferences: preferences.clone(),
        }));

        *g_save_json.lock().unwrap() = save.clone();
    };

    // Subtitle of headerbar
    let header: HeaderBar = builder.object("headerbar").expect("Could not get the headerbar");

    header.set_subtitle(Some("CachyOS rolling"));

    // Load images
    let logo_path = format!("{}/{}.svg", preferences["logo_path"].as_str().unwrap(), APP_ID);
    if Path::new(&logo_path).exists() {
        let logo = Pixbuf::from_file(logo_path).unwrap();
        main_window.set_icon(Some(&logo));
    }

    let social_box: gtk::Box = builder.object("social").unwrap();
    for btn in social_box.children() {
        let name = btn.widget_name();
        let icon_path = format!("{}/data/img/{}.png", PKGDATADIR, name);
        let image: gtk::Image = builder.object(name.as_str()).unwrap();
        image.set_from_file(Some(&icon_path));
    }

    let homepage_grid: gtk::Grid = builder.object("homepage").unwrap();
    for widget in homepage_grid.children() {
        let casted_widget = widget.downcast::<gtk::Button>();
        if casted_widget.is_err() {
            continue;
        }

        let btn = casted_widget.unwrap();
        if btn.image_position() != gtk::PositionType::Right {
            continue;
        }
        let image_path = format!("{}/data/img/external-link.png", PKGDATADIR);
        let image = gtk::Image::new();
        image.set_from_file(Some(&image_path));
        image.set_margin_start(2);
        btn.set_image(Some(&image));
    }

    // Create pages
    let pages =
        format!("{}/data/pages/{}", PKGDATADIR, preferences["default_locale"].as_str().unwrap());

    for page in fs::read_dir(pages).unwrap() {
        let scrolled_window =
            gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);

        let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        viewport.set_border_width(10);

        let label = gtk::Label::new(None);
        label.set_line_wrap(true);
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
        grid.attach(&back_btn, 0, 1, 1, 1);
        grid.attach(&label, 1, 2, 1, 1);
        viewport.add(&grid);
        scrolled_window.add(&viewport);
        scrolled_window.show_all();

        let stack: gtk::Stack = builder.object("stack").unwrap();
        let child_name =
            format!("{}page", page.unwrap().path().file_name().unwrap().to_str().unwrap());
        stack.add_named(&scrolled_window, &child_name);
    }

    // Init translation
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
        .expect("Unable to switch to the text domain.");
    gettextrs::bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set domain encoding.");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain.");
    let languages: gtk::ComboBoxText = builder.object("languages").unwrap();
    languages.set_active_id(Some(get_best_locale(&preferences, &save).as_str()));

    // Set autostart switcher state
    let autostart = Path::new(&fix_path(preferences["autostart_path"].as_str().unwrap())).exists();
    let autostart_switch: gtk::Switch = builder.object("autostart").unwrap();
    autostart_switch.set_active(autostart);

    // Live systems
    if (Path::new(&preferences["live_path"].as_str().unwrap()).exists())
        && (check_regular_file(preferences["installer_path"].as_str().unwrap()))
    {
        let installlabel: gtk::Label = builder.object("installlabel").unwrap();
        installlabel.set_visible(true);

        let install: gtk::Button = builder.object("install").unwrap();
        install.set_visible(true);

        // Show the UI
        main_window.show();
        return;
    } else {
        let installlabel: gtk::Label = builder.object("installlabel").unwrap();
        installlabel.set_visible(false);

        let install: gtk::Button = builder.object("install").unwrap();
        install.set_visible(false);
    }
    pages::create_appbrowser_page(&builder);
    pages::create_tweaks_page(&builder);

    // Show the UI
    main_window.show();
}

/// Returns the best locale, based on user's preferences.
pub fn get_best_locale(preferences: &serde_json::Value, save: &serde_json::Value) -> String {
    let saved_locale =
        format!("{}/{}/LC_MESSAGES/cachyos-hello.mo", LOCALEDIR, save["locale"].as_str().unwrap());
    if check_regular_file(saved_locale.as_str()) {
        return String::from(save["locale"].as_str().unwrap());
    } else if save["locale"].as_str().unwrap() == preferences["default_locale"].as_str().unwrap() {
        return String::from(preferences["default_locale"].as_str().unwrap());
    }

    let locale_name = std::env::var("LC_ALL").unwrap_or_else(|_| String::from("en_US.UTF-8"));
    let sys_locale =
        string_substr(locale_name.as_str(), 0, locale_name.find('.').unwrap_or(usize::MAX))
            .unwrap();
    let user_locale = format!("{}/{}/LC_MESSAGES/cachyos-hello.mo", LOCALEDIR, sys_locale);
    let two_letters = string_substr(sys_locale, 0, 2).unwrap();

    // If user's locale is supported
    if check_regular_file(user_locale.as_str()) {
        if sys_locale.contains('_') {
            return sys_locale.replace('_', "-");
        }
        return String::from(sys_locale);
    }
    // If two first letters of user's locale is supported (ex: en_US -> en)
    else if check_regular_file(
        format!("{}/{}/LC_MESSAGES/cachyos-hello.mo", LOCALEDIR, two_letters).as_str(),
    ) {
        return String::from(two_letters);
    }

    String::from(preferences["default_locale"].as_str().unwrap())
}

/// Sets locale of ui and pages.
fn set_locale(use_locale: &str) {
    if PROFILE == "Devel" {
        println!(
            "┌{0:─^40}┐\n│{1: ^40}│\n└{0:─^40}┘",
            "",
            format!("Locale changed to {}", use_locale)
        );
    }

    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain.");
    glib::setenv("LANGUAGE", use_locale, true).expect("Unable to change env variable.");

    unsafe {
        g_save_json.lock().unwrap()["locale"] = json!(use_locale);
    }

    // Real-time locale changing
    let elts: HashMap<String, serde_json::Value> = serde_json::from_str(&serde_json::to_string(&json!({
        "label": ["autostartlabel", "development", "software", "donate", "firstcategory", "forum", "install", "installlabel", "involved", "mailling", "readme", "release", "secondcategory", "thirdcategory", "welcomelabel", "welcometitle", "wiki"],
        "tooltip_text": ["about", "development", "software", "donate", "forum", "mailling", "wiki"],
    })).unwrap()).unwrap();

    let mut default_texts = json!(null);
    for method in elts.iter() {
        if default_texts.get(method.0).is_none() {
            default_texts[method.0] = json![null];
        }

        for elt in elts[method.0].as_array().unwrap() {
            let elt_value = elt.as_str().unwrap();
            unsafe {
                let item: gtk::Widget =
                    g_hello_window.clone().unwrap().builder.object(elt_value).unwrap();
                if default_texts[method.0].get(elt_value).is_none() {
                    let item_buf = item.property::<String>(method.0.as_str());
                    default_texts[method.0][elt_value] = json!(item_buf);
                }
                if method.0 == "tooltip_text" {
                    item.set_property(
                        method.0,
                        &gettextrs::gettext(default_texts[method.0][elt_value].as_str().unwrap()),
                    );
                }
            }
        }
    }

    unsafe {
        let preferences = &g_hello_window.clone().unwrap().preferences;
        let save = &*g_save_json.lock().unwrap();

        // Change content of pages
        let pages = format!(
            "{}/data/pages/{}",
            PKGDATADIR,
            preferences["default_locale"].as_str().unwrap()
        );
        for page in fs::read_dir(pages).unwrap() {
            let stack: gtk::Stack =
                g_hello_window.clone().unwrap().builder.object("stack").unwrap();
            let child = stack.child_by_name(&format!(
                "{}page",
                page.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap()
            ));
            if child.is_none() {
                eprintln!("child not found");
                continue;
            }
            let first_child = &child.unwrap().downcast::<gtk::Container>().unwrap().children();
            let second_child =
                &first_child[0].clone().downcast::<gtk::Container>().unwrap().children();
            let third_child =
                &second_child[0].clone().downcast::<gtk::Container>().unwrap().children();

            let label = &third_child[0].clone().downcast::<gtk::Label>().unwrap();
            label.set_markup(
                get_page(
                    page.unwrap().path().file_name().unwrap().to_str().unwrap(),
                    preferences,
                    save,
                )
                .as_str(),
            );
        }
    }
}

fn set_autostart(autostart: bool) {
    let autostart_path: String;
    let desktop_path: String;
    unsafe {
        autostart_path = fix_path(
            g_hello_window.clone().unwrap().preferences["autostart_path"].as_str().unwrap(),
        );
        desktop_path = g_hello_window.clone().unwrap().preferences["desktop_path"]
            .as_str()
            .unwrap()
            .to_string();
    }
    let config_dir = Path::new(&autostart_path).parent().unwrap();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).unwrap();
    }
    if autostart && !check_regular_file(autostart_path.as_str()) {
        std::os::unix::fs::symlink(desktop_path, autostart_path).unwrap();
    } else if !autostart && check_regular_file(autostart_path.as_str()) {
        std::fs::remove_file(autostart_path).unwrap();
    }
}

#[inline]
fn get_page(name: &str, preferences: &serde_json::Value, save: &serde_json::Value) -> String {
    let mut filename =
        format!("{}/data/pages/{}/{}", PKGDATADIR, save["locale"].as_str().unwrap(), name);
    if !check_regular_file(filename.as_str()) {
        filename = format!(
            "{}/data/pages/{}/{}",
            PKGDATADIR,
            preferences["default_locale"].as_str().unwrap(),
            name
        );
    }

    fs::read_to_string(filename).unwrap()
}

/// Handlers
fn on_languages_changed(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::ComboBox>().unwrap();
    let active_id = widget.active_id().unwrap();

    set_locale(active_id.as_str());

    None
}

fn on_action_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    return match widget.widget_name().as_str() {
        "install" => {
            quick_message("Calamares install type");
            None
        },
        "autostart" => {
            let action = widget.downcast::<gtk::Switch>().unwrap();
            set_autostart(action.is_active());
            None
        },
        _ => {
            show_about_dialog();
            None
        },
    };
}

fn on_btn_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Button>().unwrap();
    let name = widget.widget_name();

    unsafe {
        let stack: gtk::Stack = g_hello_window.clone().unwrap().builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{}page", name));
    };

    None
}

fn on_link_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    unsafe {
        let preferences = &g_hello_window.clone().unwrap().preferences["urls"];

        let uri = preferences[name.as_str()].as_str().unwrap();
        let _ = gtk::show_uri_on_window(gtk::Window::NONE, uri, 0);
    }

    None
}

fn on_link1_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    unsafe {
        let preferences = &g_hello_window.clone().unwrap().preferences["urls"];

        let uri = preferences[name.as_str()].as_str().unwrap();
        let _ = gtk::show_uri_on_window(gtk::Window::NONE, uri, 0);
    }

    Some(false.to_value())
}

fn on_delete_window(_param: &[glib::Value]) -> Option<glib::Value> {
    unsafe {
        let preferences = &g_hello_window.clone().unwrap().preferences["save_path"];
        let save = &*g_save_json.lock().unwrap();
        write_json(preferences.as_str().unwrap(), save);
    }

    Some(false.to_value())
}
