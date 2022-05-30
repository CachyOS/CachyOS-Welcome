use gettextrs::gettext;
use gtk::gio;
use gtk::glib::{self, clone, WeakRef};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::OnceCell;

use std::path::Path;

use crate::config::{APP_ID, PKGDATADIR, VERSION};
use crate::widgets::MainWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Application {
        pub window: OnceCell<WeakRef<MainWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        type Type = super::Application;

        const NAME: &'static str = "CachyOSHello";
    }

    impl ObjectImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self, app: &Self::Type) {
            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.show();
                window.present();
                return;
            }

            let window = MainWindow::new(app);
            self.window.set(window.downgrade()).expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            self.parent_startup(app);
            gtk::Window::set_default_icon_name(APP_ID);
        }
    }

    impl GtkApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
        .expect("Failed to create Application.")
    }

    fn private(&self) -> &imp::Application {
        imp::Application::from_instance(self)
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialog::builder()
            .transient_for(&self.main_window())
            .modal(true)
            .program_name(&gettext("Kooha"))
            .comments(&gettext("Elegantly record your screen"))
            .version(VERSION)
            .logo_icon_name(APP_ID)
            .authors(vec![
                "Dave Patrick".into(),
                "".into(),
                "Mathiascode".into(),
                "Felix Weilbach".into(),
            ])
            // Translators: Replace "translator-credits" with your names. Put a comma between.
            .translator_credits(&gettext("translator-credits"))
            .copyright(&gettext("Copyright 2021 Dave Patrick"))
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/SeaDve/Kooha")
            .website_label(&gettext("GitHub"))
            .build();

        dialog.show();
    }

    pub fn main_window(&self) -> MainWindow {
        let imp = self.private();
        imp.window.get().unwrap().upgrade().unwrap()
    }

    pub fn run(&self) {
        ApplicationExtManual::run(self);
    }
}

impl Default for Application {
    fn default() -> Self {
        gio::Application::default().unwrap().downcast().unwrap()
    }
}
