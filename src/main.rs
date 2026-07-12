#![allow(non_upper_case_globals)]
#![allow(clippy::arc_with_non_send_sync)]

mod actions;
mod cli;
mod cli_handler;
mod config;
mod dns;
mod embed_data;
mod gresource;
mod gui;
mod installer;
mod kwin_dbus;
mod localization;
mod logger;
mod networkmanager;
mod pages;
mod systemd_units;
mod tweak;
mod ui;
mod utils;
mod window;

use config::{APP_ID, PROFILE};
use utils::{PacmanWrapper, check_regular_file, fix_path, read_json, write_json};
use window::HelloWindow;

use std::path::Path;
use std::str;
use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use gtk::gio::prelude::*;
use gtk::prelude::*;

use clap::Parser;
use gtk::glib;
use i18n_embed::DesktopLanguageRequester;
use serde_json::json;
use tracing::{debug, error};
use unic_langid::LanguageIdentifier;

const RESPREFIX: &str = "/org/cachyos/hello";

static G_SAVE_JSON: LazyLock<Mutex<serde_json::Value>> = LazyLock::new(|| {
    let preferences = get_preferences();
    let saved_json = get_saved_json(&preferences);
    Mutex::new(saved_json)
});
static G_HELLO_WINDOW: OnceLock<Arc<HelloWindow>> = OnceLock::new();

fn get_saved_locale() -> Option<String> {
    let saved_json = &*G_SAVE_JSON.lock().unwrap();
    Some(saved_json["locale"].as_str()?.to_owned())
}

fn default_saved_json() -> serde_json::Value {
    json!({"locale": ""})
}

fn get_saved_json(preferences: &serde_json::Value) -> serde_json::Value {
    let save_path = fix_path(preferences["save_path"].as_str().unwrap());
    if !Path::new(&save_path).exists() {
        return default_saved_json();
    }

    match read_json(&save_path) {
        Ok(v) => v,
        _ => default_saved_json(),
    }
}

fn get_preferences() -> serde_json::Value {
    let pref_file = crate::embed_data::get("preferences.json").unwrap();
    let pref = std::str::from_utf8(pref_file.data.as_ref()).unwrap();
    serde_json::from_str(pref).expect("Unable to parse")
}

fn main() {
    // Setup logger.
    let _guard = logger::setup_logger();

    // Setup localization.
    let saved_locale = get_saved_locale().unwrap();
    let requested_languages = if saved_locale.is_empty() {
        DesktopLanguageRequester::requested_languages()
    } else {
        let lang_id: LanguageIdentifier = saved_locale.parse().unwrap();
        vec![lang_id]
    };

    let localizer = crate::localization::localizer();
    if let Err(error) = localizer.select(&requested_languages) {
        error!("Error while loading languages for library_fluent {error}");
    }

    if std::env::args().len() > 1 {
        // Parse arguments and run CLI logic
        let cli_args = cli::Cli::parse();
        if let Err(e) = run_cli(cli_args) {
            eprintln!("Error: {e}");
        }
    } else {
        // Register UI.
        gtk::init().expect("Unable to start GTK3.");

        gresource::init().expect("Could not load gresource file.");

        // Set program name.
        glib::set_program_name("org.cachyos.hello".into());
        glib::set_application_name("org.cachyos.hello");

        let application = gtk::Application::new(
            Some(APP_ID),       // Application id
            Default::default(), // Using default flags
        );

        application.connect_activate(|application| {
            // If a window already exists, raise the window to the front and give it focus
            if let Some(window) = G_HELLO_WINDOW.get() {
                window.window.present();
                return;
            }
            build_ui(application);
        });

        // Run the application and start the event loop
        application.run();
    }
}

fn build_ui(application: &gtk::Application) {
    let preferences = get_preferences();

    // Get saved infos
    let saved_locale = get_saved_locale().unwrap();

    // Detect best locale
    let best_locale =
        get_best_locale(&preferences, &saved_locale).expect("Failed to get best locale");

    // Init window
    let hello_window = HelloWindow::new(application, preferences, &best_locale);

    let builder_ref = &hello_window.builder;
    builder_ref.connect_signals(|_builder, handler_name| {
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
    G_SAVE_JSON.lock().unwrap()["locale"] = json!(best_locale);

    G_HELLO_WINDOW.set(Arc::new(hello_window)).unwrap();
}

/// Returns the best locale, based on user's preferences.
fn get_best_locale(
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
        utils::string_substr(locale_name.as_str(), 0, locale_name.find('.').unwrap_or(usize::MAX))?;
    let two_letters = utils::string_substr(sys_locale, 0, 2)?;

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
        debug!("┌{0:─^40}┐\n│{1: ^40}│\n└{0:─^40}┘", "", format!("Locale changed to {use_locale}"));
    }

    // change UI
    G_HELLO_WINDOW.get().unwrap().switch_locale(use_locale);

    // save changes
    G_SAVE_JSON.lock().unwrap()["locale"] = json!(use_locale);
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
    match widget.widget_name().as_str() {
        "install" => {
            installer::launch_installer(fl!("calamares-install-type"));
            None
        },
        "autostart" => {
            let action = widget.downcast::<gtk::Switch>().unwrap();
            G_HELLO_WINDOW.get().unwrap().set_autostart(action.is_active());
            None
        },
        _ => {
            G_HELLO_WINDOW.get().unwrap().show_about_dialog();
            None
        },
    }
}

fn on_btn_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Button>().unwrap();
    let name = widget.widget_name();

    let child_name = format!("{name}page");
    G_HELLO_WINDOW.get().unwrap().set_stack_child_visible(&child_name);

    None
}

fn on_link_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    let preferences = G_HELLO_WINDOW.get().unwrap().get_preferences("urls");

    let uri = preferences[name.as_str()].as_str().unwrap();
    G_HELLO_WINDOW.get().unwrap().open_uri(uri);

    None
}

fn on_link1_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    let preferences = G_HELLO_WINDOW.get().unwrap().get_preferences("urls");

    let uri = preferences[name.as_str()].as_str().unwrap();
    G_HELLO_WINDOW.get().unwrap().open_uri(uri);

    Some(false.to_value())
}

fn on_delete_window(_param: &[glib::Value]) -> Option<glib::Value> {
    let saved_json = &*G_SAVE_JSON.lock().unwrap();
    let preferences = G_HELLO_WINDOW.get().unwrap().get_preferences("save_path");
    if let Err(e) = write_json(preferences.as_str().unwrap(), saved_json) {
        error!("Could not save settings: {e:?}");
    }

    Some(false.to_value())
}

fn run_cli(cli: cli::Cli) -> anyhow::Result<()> {
    match cli.command {
        cli::Commands::Fix(args) => cli_handler::handle_fix_command(args.action),
        cli::Commands::Tweak(args) => cli_handler::handle_tweak_command(args.action),
        cli::Commands::Dns(args) => cli_handler::handle_dns_command(args.action),
        cli::Commands::Launch(args) => cli_handler::handle_launch_command(args.app),
    }
}
