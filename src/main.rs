#![feature(const_str_from_utf8)]
#![feature(string_remove_matches)]
#![allow(non_upper_case_globals)]

mod alpm_helper;
mod application_browser;
mod config;
mod data_types;
mod embed_data;
mod gresource;
mod localization;
mod pages;
mod utils;

use config::{APP_ID, PROFILE, VERSION};
use data_types::*;
use glib::GString;
use gtk::{gio, glib, Builder, HeaderBar, Window};
use i18n_embed::DesktopLanguageRequester;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use unic_langid::LanguageIdentifier;
use utils::*;

use gio::prelude::*;
use gtk::prelude::*;

use gdk_pixbuf::Pixbuf;

use serde_json::json;
use std::{fs, str};
use subprocess::Exec;

const RESPREFIX: &str = "/org/cachyos/hello";

static G_SAVE_JSON: Lazy<Mutex<serde_json::Value>> = Lazy::new(|| {
    let saved_json = get_saved_json();
    Mutex::new(saved_json)
});
static mut G_HELLO_WINDOW: Option<Arc<HelloWindow>> = None;

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
    let main_window: &Window = unsafe { G_HELLO_WINDOW.as_ref().unwrap().window.as_ref() };
    let logo_path = format!("/usr/share/icons/hicolor/scalable/apps/{APP_ID}.svg");
    let logo = Pixbuf::from_file(logo_path).unwrap();

    let dialog = gtk::AboutDialog::builder()
        .transient_for(main_window)
        .modal(true)
        .program_name(GString::from_string_unchecked(crate::fl!("about-dialog-title")))
        .comments(GString::from_string_unchecked(crate::fl!("about-dialog-comments")))
        .version(VERSION)
        .logo(&logo)
        .authors(vec![
            "Vladislav Nepogodin".to_owned(),
        ])
        // Translators: Replace "translator-credits" with your names. Put a comma between.
        .translator_credits("translator-credits")
        .copyright("2021-2023 CachyOS team")
        .license_type(gtk::License::Gpl30)
        .website("https://github.com/cachyos/cachyos-welcome")
        .website_label("GitHub")
        .build();

    dialog.run();
    dialog.hide();
}

fn get_preferences() -> serde_json::Value {
    let page_file = crate::embed_data::get("preferences.json").unwrap();
    let page = std::str::from_utf8(page_file.data.as_ref());
    serde_json::from_str(page.unwrap()).expect("Unable to parse")
}

fn get_saved_locale() -> Option<String> {
    let saved_json = &*G_SAVE_JSON.lock().unwrap();
    Some(saved_json["locale"].as_str()?.to_owned())
}

fn get_saved_json() -> serde_json::Value {
    let preferences = get_preferences();
    let save_path = fix_path(preferences["save_path"].as_str().unwrap());
    if !Path::new(&save_path).exists() {
        json!({"locale": ""})
    } else {
        read_json(save_path.as_str())
    }
}

fn main() {
    // Setup localization.
    let saved_locale = get_saved_locale().unwrap();
    let requested_languages = if !saved_locale.is_empty() {
        let lang_id: LanguageIdentifier = saved_locale.parse().unwrap();
        vec![lang_id]
    } else {
        DesktopLanguageRequester::requested_languages()
    };

    let localizer = crate::localization::localizer();
    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!("Error while loading languages for library_fluent {}", error);
    }

    // Register UI.
    gtk::init().expect("Unable to start GTK3.");

    gresource::init().expect("Could not load gresource file.");

    // Set program name.
    glib::set_program_name("CachyOSHello".into());
    glib::set_application_name("CachyOSHello");

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
    let preferences = get_preferences();

    // Get saved infos
    let saved_locale = get_saved_locale().unwrap();
    let best_locale = get_best_locale(&preferences, &saved_locale).unwrap();

    // Import Css
    let provider = gtk::CssProvider::new();
    provider.load_from_resource(&format!("{RESPREFIX}/ui/style.css"));
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Init window
    let builder: Builder = Builder::from_resource(&format!("{RESPREFIX}/ui/cachyos-hello.glade"));
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
        G_HELLO_WINDOW = Some(Arc::new(HelloWindow {
            window: main_window.clone(),
            builder: builder.clone(),
            preferences: preferences.clone(),
        }));
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
        let icon_path = format!("{RESPREFIX}/data/img/{name}.png");
        let image: gtk::Image = builder.object(name.as_str()).unwrap();
        image.set_from_resource(Some(&icon_path));
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
        let image_path = format!("{RESPREFIX}/data/img/external-link.png");
        let image = gtk::Image::new();
        image.set_from_resource(Some(&image_path));
        image.set_margin_start(2);
        btn.set_image(Some(&image));
    }

    // Create pages
    let file_pages_path = crate::embed_data::HelloData::iter()
        .filter(|pkg| pkg.starts_with(&format!("pages/{}", &best_locale)))
        .collect::<Vec<_>>();

    for file_path in file_pages_path {
        // let page_file = HelloData::get(&file_path).unwrap();
        // let page = std::str::from_utf8(page_file.data.as_ref());
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
            stack.set_visible_child_name(&format!("{name}page"));
        }));

        let child_name = format!(
            "{}page",
            Path::new(&file_path.as_ref()).file_name().unwrap().to_str().unwrap()
        );

        let grid = gtk::Grid::new();
        grid.set_widget_name(&child_name);
        grid.attach(&back_btn, 0, 1, 1, 1);
        grid.attach(&label, 1, 2, 1, 1);
        viewport.add(&grid);
        scrolled_window.add(&viewport);
        scrolled_window.show_all();

        let stack: gtk::Stack = builder.object("stack").unwrap();
        stack.add_named(&scrolled_window, &child_name);
    }

    // Init translation
    let languages: gtk::ComboBoxText = builder.object("languages").unwrap();
    languages.set_active_id(Some(best_locale.as_str()));

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
pub fn get_best_locale(
    preferences: &serde_json::Value,
    saved_locale: &str,
) -> Result<String, str::Utf8Error> {
    if crate::localization::check_language_valid(saved_locale) {
        return Ok(saved_locale.to_owned());
    } else if saved_locale == preferences["default_locale"].as_str().unwrap() {
        return Ok(preferences["default_locale"].as_str().unwrap().to_owned());
    }

    let locale_name = crate::localization::get_default_lang();
    let sys_locale =
        string_substr(locale_name.as_str(), 0, locale_name.find('.').unwrap_or(usize::MAX))?;
    let two_letters = string_substr(sys_locale, 0, 2)?;

    // If user's locale is supported
    if crate::localization::check_language_valid(sys_locale) {
        if sys_locale.contains('_') {
            return Ok(sys_locale.replace('_', "-"));
        }
        return Ok(sys_locale.to_owned());
    }
    // If two first letters of user's locale is supported (ex: en_US -> en)
    else if crate::localization::check_language_valid(two_letters) {
        return Ok(two_letters.to_owned());
    }

    Ok(preferences["default_locale"].as_str().unwrap().to_owned())
}

/// Sets locale of ui and pages.
fn set_locale(use_locale: &str) {
    if PROFILE == "Devel" {
        println!(
            "┌{0:─^40}┐\n│{1: ^40}│\n└{0:─^40}┘",
            "",
            format!("Locale changed to {use_locale}")
        );
    }

    let localizer = crate::localization::localizer();
    let req_locale: LanguageIdentifier = use_locale.parse().unwrap();

    if let Err(error) = localizer.select(&[req_locale]) {
        eprintln!("Error while loading languages for library_fluent {}", error);
    }

    G_SAVE_JSON.lock().unwrap()["locale"] = json!(use_locale);

    // Run-time locale changing
    let elts: HashMap<&str, Vec<_>> = HashMap::from([
        ("label", vec![
            "autostartlabel",
            "development",
            "software",
            "donate",
            "firstcategory",
            "forum",
            "install",
            "installlabel",
            "involved",
            "readme",
            "release",
            "secondcategory",
            "thirdcategory",
            "welcomelabel",
            "welcometitle",
            "wiki",
        ]),
        ("tooltip_text", vec!["about", "development", "software", "donate", "forum", "wiki"]),
    ]);

    let builder_ref = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().builder };

    for (method, objnames) in &elts {
        for objname in objnames {
            let item: &gtk::Widget = &builder_ref.object(objname).unwrap();
            if method == &"label" {
                let translated_text =
                    crate::localization::get_locale_text(utils::get_translation_msgid(objname));
                item.set_property(method, &translated_text);
            } else if method == &"tooltip_text" {
                let translated_text = if objname == &"about" {
                    crate::fl!("button-about-tooltip")
                } else {
                    crate::fl!("button-web-resource-tooltip")
                };
                item.set_property(method, &translated_text);
            }
        }
    }

    // Change content of pages
    let file_pages_path = crate::embed_data::HelloData::iter()
        .filter(|pkg| pkg.starts_with(&format!("pages/{}", &use_locale)))
        .collect::<Vec<_>>();

    for file_path in file_pages_path {
        let page_file_name = Path::new(file_path.as_ref()).file_name().unwrap().to_str().unwrap();

        let stack: &gtk::Stack = &builder_ref.object("stack").unwrap();
        let child = stack.child_by_name(&format!("{}page", &page_file_name));
        if child.is_none() {
            eprintln!("child not found");
            continue;
        }
        let first_child = &child.unwrap().downcast::<gtk::Container>().unwrap().children();
        let second_child = &first_child[0].clone().downcast::<gtk::Container>().unwrap().children();
        let third_child = &second_child[0].clone().downcast::<gtk::Container>().unwrap().children();

        let label = &third_child[0].clone().downcast::<gtk::Label>().unwrap();
        label.set_markup(get_page(file_path.as_ref()).as_str());
    }

    pages::update_translations(builder_ref);
}

fn set_autostart(autostart: bool) {
    let preferences = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().preferences };
    let autostart_path = fix_path(preferences["autostart_path"].as_str().unwrap());
    let desktop_path = preferences["desktop_path"].as_str().unwrap().to_owned();
    let config_dir = Path::new(&autostart_path).parent().unwrap();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).unwrap();
    }
    if autostart && !check_regular_file(&autostart_path) {
        std::os::unix::fs::symlink(desktop_path, &autostart_path).unwrap();
    } else if !autostart && check_regular_file(&autostart_path) {
        std::fs::remove_file(&autostart_path).unwrap();
    }
}

#[inline]
fn get_page(file_path: &str) -> String {
    let page_file = crate::embed_data::get(file_path).unwrap();
    let page = std::str::from_utf8(page_file.data.as_ref());
    page.unwrap().to_owned()
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

    let builder_ref = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().builder };
    let stack: &gtk::Stack = &builder_ref.object("stack").unwrap();
    stack.set_visible_child_name(&format!("{name}page"));

    None
}

fn on_link_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    let window_ref = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().window };
    let preferences = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().preferences["urls"] };

    let uri = preferences[name.as_str()].as_str().unwrap();
    let _ = gtk::show_uri_on_window(Some(window_ref), uri, 0);

    None
}

fn on_link1_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    let window_ref = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().window };
    let preferences = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().preferences["urls"] };

    let uri = preferences[name.as_str()].as_str().unwrap();
    let _ = gtk::show_uri_on_window(Some(window_ref), uri, 0);

    Some(false.to_value())
}

fn on_delete_window(_param: &[glib::Value]) -> Option<glib::Value> {
    let saved_json = &*G_SAVE_JSON.lock().unwrap();
    let preferences = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().preferences["save_path"] };
    write_json(preferences.as_str().unwrap(), saved_json);

    Some(false.to_value())
}
