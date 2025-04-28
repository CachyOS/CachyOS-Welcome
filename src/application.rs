use crate::config::APP_ID;
// Remove GTK imports
// use crate::window::HelloWindow;
// use gio::prelude::*;
// use glib::ExitCode;
// use gtk::prelude::*;
// use gtk::{Application, Builder};

use crate::localization::localizer;
use i18n_embed::{DesktopLanguageRequester, I18nEmbedError};
use unic_langid::LanguageIdentifier;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{debug, error};
use serde_json::Value;

// Keep preference loading logic if it's complex and called from main.rs
// Otherwise, this logic can be moved directly into main.rs/AppModel::default()

pub fn load_preferences_from_application() -> Result<Value, anyhow::Error> {
    let sys_path = "/etc/cachyos-hello/cachyos-hello.json";
    let share_path = "/usr/share/cachyos-hello/cachyos-hello.json";
    let user_path = "~/.config/cachyos-hello/cachyos-hello.json";

    let path_to_try = if Path::new(sys_path).exists() {
        sys_path.to_string()
    } else if Path::new(share_path).exists() {
        share_path.to_string()
    } else {
        crate::utils::fix_path(user_path) // Use utils function
    };

    debug!("(from application.rs) Loading preferences from: {}", path_to_try);
    let mut f = File::open(path_to_try)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let prefs: Value = serde_json::from_str(&buffer)?;
    Ok(prefs)
}

// This file might become obsolete if all state moves to AppModel in main.rs
// For now, keep the basic structure for preference loading if used by AppModel setup.

// Define a struct to hold application state, replacing the GTK Application part
/*
pub struct CachyApplication {
    // Add fields needed for Qt application logic
    // For example: preferences, current_locale
    pub preferences: serde_json::Value,
    pub current_locale: String,
}

impl CachyApplication {
    pub fn new() -> Self {
        let preferences = load_preferences_from_application().unwrap_or_default(); // Handle error better
        // Locale determination is done in AppModel::default()
        CachyApplication {
            preferences,
            current_locale: "en".to_string(), // Placeholder
        }
    }

    // This run method will be replaced by Qt main loop setup in main.rs
    // pub fn run(&self) -> ExitCode {
    //     let application = Application::new(Some(APP_ID), Default::default());
    //
    //     application.connect_activate(move |app| {
    //         let window = HelloWindow::new(app, self.preferences.clone(), &self.current_locale);
    //         let builder = window.builder.clone();
    //         let app_clone = app.clone();
    //         let window_clone = window.window.clone();
    //
    //         // Attach signal handlers (This logic moves to the QObject/QML)
    //         crate::attach_callbacks(&builder, &window_clone, &app_clone);
    //     });
    //
    //     application.run()
    // }
}
*/

// Remove GTK signal handlers (these callbacks need to be reimplemented as QInvokable methods)
/*
fn attach_callbacks(builder: &Builder, window: &gtk::Window, _application: &Application) {
    let preferences = window.clone().downcast::<HelloWindow>().unwrap().preferences;

    let stack: gtk::Stack = builder.object("stack").unwrap();
    stack.connect_visible_child_name_notify(glib::clone!(@weak window => move |_| {
        let stack: gtk::Stack = builder.object("stack").unwrap();
        if stack.visible_child_name().unwrap().as_str() == "homepage" {
            window.set_title("CachyOS Hello");
        } else {
            window.set_title("CachyOS Hello - Documentation");
        }
    }));

    let homepage_grid: gtk::Grid = builder.object("homepage").unwrap();
    for widget in homepage_grid.children() {
        if let Ok(button) = widget.downcast::<gtk::Button>() {
            let name = button.widget_name();
            let handler = match name.as_str() {
                "readme" | "release" | "involved" | "appBrowser" | "tweaksBrowser" => "on_btn_clicked",
                "wiki" | "forum" | "software" | "development" | "donate" => "on_link_clicked",
                _ => continue,
            };
            match handler {
                "on_btn_clicked" => {
                    button.connect_clicked(glib::clone!(@weak window => move |button| {
                        let name = button.widget_name();
                        let hello_window = window.downcast_ref::<HelloWindow>().unwrap();
                        hello_window.set_stack_child_visible(&format!("{}page", name));
                    }));
                }
                "on_link_clicked" => {
                    button.connect_clicked(glib::clone!(@weak window => move |button| {
                        let hello_window = window.downcast_ref::<HelloWindow>().unwrap();
                        let name = button.widget_name();
                        let url = hello_window.get_preferences("url")[name.as_str()].as_str().unwrap();
                        hello_window.open_uri(url);
                    }));
                }
                _ => {}
            }
        }
    }

    let social_box: gtk::Box = builder.object("social").unwrap();
    for widget in social_box.children() {
        if let Ok(eventbox) = widget.downcast::<gtk::EventBox>() {
            let name = eventbox.widget_name();
            eventbox.connect_button_press_event(glib::clone!(@weak window => move |_,_| {
                let hello_window = window.downcast_ref::<HelloWindow>().unwrap();
                let url = hello_window.get_preferences("url")[name.as_str()].as_str().unwrap();
                hello_window.open_uri(url);
                gtk::Inhibit(true)
            }));
        }
    }

    let languages: gtk::ComboBoxText = builder.object("languages").unwrap();
    languages.connect_changed(glib::clone!(@weak window => move |combobox| {
        let hello_window = window.downcast_ref::<HelloWindow>().unwrap();
        let locale_id = combobox.active_id().unwrap();
        hello_window.switch_locale(locale_id.as_str());
    }));

    let actions = ["install", "about", "autostart"];
    for action in actions {
        let item = builder.object::<gtk::Widget>(action).unwrap();
        match action {
            "install" | "about" => {
                item.downcast::<gtk::Button>().unwrap().connect_clicked(glib::clone!(@weak window => move |button| {
                    let hello_window = window.downcast_ref::<HelloWindow>().unwrap();
                    match button.widget_name().as_str() {
                        "install" => crate::installer::run_installer(&hello_window.preferences),
                        "about" => hello_window.show_about_dialog(),
                        _ => {}
                    }
                }));
            }
            "autostart" => {
                item.downcast::<gtk::Switch>().unwrap().connect_state_set(glib::clone!(@weak window => move |_, state| {
                    let hello_window = window.downcast_ref::<HelloWindow>().unwrap();
                    hello_window.set_autostart(state);
                    gtk::Inhibit(false)
                }));
            }
            _ => unreachable!(),
        }
    }
}
*/
