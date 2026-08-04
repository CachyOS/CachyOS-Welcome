use crate::config::{APP_ID, VERSION};
use crate::{RESPREFIX, check_regular_file, installer, pages, utils};

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use gtk::prelude::*;

use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::GString;
use gtk::{Builder, HeaderBar, Window, gdk, glib};
use tracing::{debug, error};
use unic_langid::LanguageIdentifier;

#[derive(Clone, Debug)]
pub struct HelloWindow {
    pub builder: gtk::Builder,
    pub window: gtk::Window,
    preferences: serde_json::Value,
}

// SAFETY: GTK objects use GLib's atomic reference counting for lifetime management.
// All GTK UI calls in this application occur on the GTK main thread via GTK's own
// signal/callback system. The static is written once at startup before any reads.
unsafe impl Send for HelloWindow {}
unsafe impl Sync for HelloWindow {}

impl HelloWindow {
    /// Create a new `HelloWindow`.
    #[expect(clippy::too_many_lines, reason = "GTK window initialization")]
    pub fn new(
        application: &gtk::Application,
        preferences: serde_json::Value,
        best_locale: &str,
    ) -> Self {
        // Register bundled icons with the icon theme
        if let Some(icon_theme) = gtk::IconTheme::default() {
            icon_theme.add_resource_path(&format!("{RESPREFIX}/data/img"));
        }

        // Import Css
        let provider = gtk::CssProvider::new();
        provider.load_from_resource(&format!("{RESPREFIX}/ui/style.css"));
        gtk::StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Init window
        let builder = Builder::from_resource(&format!("{RESPREFIX}/ui/cachyos-hello.glade"));

        let main_window: Window =
            builder.object("window").expect("Could not get the object window");
        main_window.set_application(Some(application));

        // Subtitle of headerbar
        let header: HeaderBar = builder.object("headerbar").expect("Could not get the headerbar");
        header.set_subtitle(Some("CachyOS rolling"));

        // Load images
        let logo_path = format!("{}/{APP_ID}.svg", preferences["logo_path"].as_str().unwrap());
        if Path::new(&logo_path).exists() {
            let logo = Pixbuf::from_file(logo_path).unwrap();
            main_window.set_icon(Some(&logo));
        }

        let social_box: gtk::Box = builder.object("social").unwrap();
        for btn in social_box.children() {
            let name = btn.widget_name();
            let image: gtk::Image = builder.object(name.as_str()).unwrap();
            image.set_pixel_size(32);
            image.set_from_icon_name(Some(name.as_str()), gtk::IconSize::Invalid);

            // Enable keyboard activation (Enter/Space) for social EventBoxes
            btn.connect_key_press_event(|widget, event| {
                let keyval = event.keyval();
                if keyval == gdk::keys::constants::Return
                    || keyval == gdk::keys::constants::KP_Enter
                    || keyval == gdk::keys::constants::space
                {
                    let name = widget.widget_name();
                    let hello_window = crate::G_HELLO_WINDOW.get().unwrap();
                    let preferences = hello_window.get_preferences("urls");
                    if let Some(uri) = preferences[name.as_str()].as_str() {
                        hello_window.open_uri(uri);
                    }
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            });
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
            let image = gtk::Image::from_icon_name(
                Some("external-link-symbolic"),
                gtk::IconSize::SmallToolbar,
            );
            image.set_margin_start(2);
            btn.set_image(Some(&image));
        }

        // Create pages
        let locale_pages_exist = crate::embed_data::HelloData::iter()
            .any(|x| x.starts_with(&format!("pages/{best_locale}")));
        let file_pages_path =
            if locale_pages_exist { format!("pages/{best_locale}") } else { "pages/en".to_owned() };

        let mut file_pages: Vec<std::borrow::Cow<'_, str>> = crate::embed_data::HelloData::iter()
            .filter(|pkg| pkg.starts_with(&file_pages_path))
            .collect();

        // Release page is only generated in English, always include it
        if file_pages_path != "pages/en"
            && let Some(en_release) =
                crate::embed_data::HelloData::iter().find(|pkg| pkg.as_ref() == "pages/en/release")
        {
            file_pages.push(en_release);
        }

        for file_path in file_pages {
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
        languages.set_active_id(Some(best_locale));

        // Set autostart switcher state
        let autostart =
            Path::new(&utils::fix_path(preferences["autostart_path"].as_str().unwrap())).exists();
        let autostart_switch: gtk::Switch = builder.object("autostart").unwrap();
        autostart_switch.set_active(autostart);

        // Live systems
        if installer::is_iso(&preferences) {
            let installlabel: gtk::Label = builder.object("installlabel").unwrap();
            installlabel.set_visible(true);

            let install: gtk::Button = builder.object("install").unwrap();
            install.set_visible(true);
        } else {
            let installlabel: gtk::Label = builder.object("installlabel").unwrap();
            installlabel.set_visible(false);

            let install: gtk::Button = builder.object("install").unwrap();
            install.set_visible(false);

            pages::create_appbrowser_page(&builder);
            pages::create_tweaks_page(&builder);
            pages::create_troubleshooting_page(&builder);

            if Path::new("/usr/bin/nmcli").exists() {
                pages::dns::create_connections_page(&builder);
            }
        }

        // Show the UI
        main_window.show();

        // Set initial focus on a meaningful interactive widget
        if installer::is_iso(&preferences) {
            let install: gtk::Button = builder.object("install").unwrap();
            install.grab_focus();
        } else {
            let readme: gtk::Button = builder.object("readme").unwrap();
            readme.grab_focus();
        }

        // Briefly dim focused widget on keyboard Tab for navigation feedback
        let prev_dimmed: Rc<RefCell<Option<gtk::Widget>>> = Rc::new(RefCell::new(None));
        let prev = prev_dimmed.clone();
        let window = main_window.clone();
        main_window.connect_key_press_event(move |_, event| {
            let keyval = event.keyval();
            if keyval == gdk::keys::constants::Tab || keyval == gdk::keys::constants::ISO_Left_Tab {
                let window = window.clone();
                let prev = prev.clone();
                glib::idle_add_local_once(move || {
                    if let Some(widget) = prev.borrow_mut().take() {
                        widget.set_opacity(1.0);
                    }
                    if let Some(focused) = window.focused_widget() {
                        focused.set_opacity(0.5);
                        *prev.borrow_mut() = Some(focused.clone());
                        glib::timeout_add_local_once(Duration::from_millis(180), move || {
                            focused.set_opacity(1.0);
                        });
                    }
                });
            }
            glib::Propagation::Proceed
        });

        // setup pages content
        let hello_window = HelloWindow { window: main_window, builder, preferences };
        hello_window.switch_locale(best_locale);
        hello_window
    }

    pub fn show_about_dialog(&self) {
        let logo_path = format!("/usr/share/icons/hicolor/scalable/apps/{APP_ID}.svg");
        let logo = Pixbuf::from_file(logo_path).unwrap();

        let dialog = gtk::AboutDialog::builder()
        .transient_for(&self.window)
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
        .copyright("2021-2026 CachyOS team")
        .license_type(gtk::License::Gpl30)
        .website("https://github.com/cachyos/cachyos-welcome")
        .website_label("GitHub")
        .build();

        dialog.run();
        dialog.hide();
    }

    /// Change languages strings of ui and pages.
    pub fn switch_locale(&self, use_locale: &str) {
        let localizer = crate::localization::localizer();
        let req_locale: LanguageIdentifier = use_locale.parse().unwrap();

        if let Err(error) = localizer.select(&[req_locale]) {
            error!("Error while loading languages for library_fluent {error}");
        }

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

        for (method, objnames) in &elts {
            for objname in objnames {
                let item: &gtk::Widget = &self.builder.object(objname).unwrap();
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
        let locale_pages_exist = crate::embed_data::HelloData::iter()
            .any(|x| x.starts_with(&format!("pages/{use_locale}")));
        let file_pages_path =
            if locale_pages_exist { format!("pages/{use_locale}") } else { "pages/en".to_owned() };

        let mut file_pages: Vec<std::borrow::Cow<'_, str>> = crate::embed_data::HelloData::iter()
            .filter(|pkg| pkg.starts_with(&file_pages_path))
            .collect();

        // Release page is only generated in English, always include it
        if file_pages_path != "pages/en"
            && let Some(en_release) =
                crate::embed_data::HelloData::iter().find(|pkg| pkg.as_ref() == "pages/en/release")
        {
            file_pages.push(en_release);
        }

        for file_path in file_pages {
            let page_file_name =
                Path::new(file_path.as_ref()).file_name().unwrap().to_str().unwrap();

            let stack: &gtk::Stack = &self.builder.object("stack").unwrap();
            let child = stack.child_by_name(&format!("{page_file_name}page"));
            if child.is_none() {
                debug!("child not found");
                continue;
            }
            let first_child = &child.unwrap().downcast::<gtk::Container>().unwrap().children();
            let second_child =
                &first_child[0].clone().downcast::<gtk::Container>().unwrap().children();
            let third_child =
                &second_child[0].clone().downcast::<gtk::Container>().unwrap().children();

            let label = &third_child[0].clone().downcast::<gtk::Label>().unwrap();
            label.set_markup(get_page(file_path.as_ref()).as_str());
        }

        pages::i18n::update_translations(&self.builder);
    }

    pub fn set_autostart(&self, autostart: bool) {
        let preferences = &self.preferences;
        let autostart_path = utils::fix_path(preferences["autostart_path"].as_str().unwrap());
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

    pub fn open_uri(&self, uri: &str) {
        if let Err(uri_err) = gtk::show_uri_on_window(Some(&self.window), uri, 0) {
            error!("Failed to open uri: {uri_err}");
        }
    }

    pub fn set_stack_child_visible(&self, child_name: &str) {
        let stack: &gtk::Stack = &self.builder.object("stack").unwrap();
        stack.set_visible_child_name(child_name);
    }

    pub fn get_preferences(&self, entry: &str) -> &serde_json::Value {
        &self.preferences[entry]
    }
}

#[inline]
fn get_page(file_path: &str) -> String {
    let page_file = crate::embed_data::get(file_path).unwrap();
    let page = std::str::from_utf8(page_file.data.as_ref());
    page.unwrap().to_owned()
}
