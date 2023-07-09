use crate::alpm_helper::*;
use crate::utils;

use gio::prelude::*;
use gtk::prelude::{
    BoxExt, ButtonExt, CellRendererExt, CellRendererTextExt, CellRendererToggleExt, ComboBoxExt,
    ContainerExt, GridExt, GtkListStoreExt, GtkListStoreExtManual, ScrolledWindowExt,
    ToggleButtonExt, TreeModelExt, TreeStoreExt, TreeStoreExtManual, TreeViewColumnExt,
    TreeViewExt, WidgetExt,
};

use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
pub struct ApplicationBrowser {
    pub alpm_handle: alpm::Alpm,
    pub alpm_helper: AlpmHelper,
    pub filter: bool,
    pub app_store: gtk::TreeStore,
    pub group_store: gtk::ListStore,
    pub group_tofilter: String,
    pub groups: serde_json::Value,
    pub tree_view: gtk::TreeView,
    pub app_browser_box: gtk::Box,
    pub button_box: gtk::Box,
    pub back_btn: gtk::Button,
    pub update_system_btn: gtk::Button,
}

fn new_alpm() -> alpm::Result<alpm::Alpm> {
    let pacman = pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
    let alpm = alpm_utils::alpm_with_conf(&pacman)?;

    Ok(alpm)
}

const GROUP: u32 = 0;
const ICON: u32 = 1;
const APPLICATION: u32 = 2;
const DESCRIPTION: u32 = 3;
const ACTIVE: u32 = 4;
const PACKAGE: u32 = 5;
const INSTALLED: u32 = 6;

static mut G_APP_BROWSER: Lazy<Mutex<ApplicationBrowser>> = Lazy::new(|| {
    let mut app_browser = ApplicationBrowser::new();
    app_browser.create_page();
    Mutex::new(app_browser)
});

impl ApplicationBrowser {
    pub fn new() -> Self {
        let app_browser_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        app_browser_box.set_expand(true);

        let child_name = "appBrowserpage";

        let back_image = gtk::Image::from_icon_name(Some("go-previous"), gtk::IconSize::Button);
        let back_btn = gtk::Button::new();
        back_btn.set_image(Some(&back_image));
        back_btn.set_widget_name("home");

        let back_grid = gtk::Grid::new();
        back_grid.set_hexpand(true);
        back_grid.set_margin_start(10);
        back_grid.set_margin_top(5);
        back_grid.attach(&back_btn, 0, 1, 1, 1);

        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        button_box.set_widget_name(child_name);
        let advanced_button = gtk::ToggleButton::with_label("advanced");
        advanced_button.set_tooltip_text(Some("Toggle an extended selection of packages"));
        advanced_button.connect_clicked(on_advanced_clicked);
        // let download_button = gtk::Button::with_label("download");
        // download_button.set_tooltip_text(Some("Download the most recent selection of packages"));
        // download_button.connect_clicked(on_download_clicked);
        let reset_button = gtk::Button::with_label("reset");
        reset_button.set_tooltip_text(Some("Reset your current selections..."));
        reset_button.connect_clicked(on_reload_clicked);
        let update_system_btn = gtk::Button::with_label("UPDATE SYSTEM");
        update_system_btn.set_tooltip_text(Some("Apply your current selections to the system"));
        update_system_btn.connect_clicked(on_update_system_clicked);
        update_system_btn.set_sensitive(false);

        // Group filter
        let app_util_manifest_file =
            crate::embed_data::get("application_utility/default.json").unwrap();
        let app_util_manifest = std::str::from_utf8(app_util_manifest_file.data.as_ref()).unwrap();
        let groups: serde_json::Value =
            serde_json::from_str(app_util_manifest).expect("Unable to parse");
        let group_store = load_groups_data(&groups);
        let group_combo = utils::create_combo_with_model(&group_store);
        group_combo.connect_changed(on_group_filter_changed);

        // Packing button box
        button_box.pack_start(&advanced_button, false, false, 10);
        button_box.pack_start(&group_combo, false, false, 10);
        button_box.pack_end(&update_system_btn, false, false, 10);

        button_box.pack_end(&reset_button, false, false, 10);
        // button_box.pack_end(&download_button, false, false, 10);
        app_browser_box.pack_start(&back_grid, false, false, 0);
        app_browser_box.pack_start(&button_box, false, false, 10);

        let col_types: [glib::Type; 7] = [
            String::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
            i32::static_type(),
            String::static_type(),
            i32::static_type(),
        ];

        Self {
            alpm_handle: new_alpm().unwrap(),
            alpm_helper: AlpmHelper::new(),
            filter: false,
            app_store: gtk::TreeStore::new(&col_types),
            group_store,
            groups,
            group_tofilter: String::from("*"),
            tree_view: gtk::TreeView::new(),
            app_browser_box,
            button_box,
            back_btn,
            update_system_btn,
        }
    }

    pub fn default_impl() -> &'static Mutex<Self> {
        unsafe { &G_APP_BROWSER }
    }

    fn load_app_data(&mut self) -> usize {
        // not use data set for the moment
        let mut store_size: usize = 0;

        let localdb = self.alpm_handle.localdb();

        for group in self.groups.as_array().unwrap() {
            if let Some(apps_map) = group.get("apps") {
                let g_name = String::from(group["name"].as_str().unwrap());
                let g_icon = String::from(group["icon"].as_str().unwrap());
                let mut g_desc = String::from(group["description"].as_str().unwrap());
                if g_desc.len() < 72 {
                    g_desc += " ";
                }

                if self.group_tofilter != "*" && self.group_tofilter != g_name {
                    continue;
                }
                if group["filter"].as_array().is_some() && !self.filter {
                    continue;
                }

                let index = self.app_store.insert_with_values(None, None, &[
                    (GROUP, &None::<String>),
                    (ICON, &g_icon),
                    (APPLICATION, &g_name),
                    (DESCRIPTION, &g_desc),
                    (ACTIVE, &-1),
                    (PACKAGE, &None::<String>),
                    (INSTALLED, &-1),
                ]);
                store_size += 1;

                for app in apps_map.as_array().unwrap() {
                    let app_name = String::from(app["pkg"].as_str().unwrap());
                    let mut status = localdb.pkg(app_name).is_ok();

                    if app["filter"].as_array().is_some() && !self.filter {
                        continue;
                    }

                    // Restore user checks
                    if !status
                        && self.alpm_helper.to_install(&String::from(app["pkg"].as_str().unwrap()))
                    {
                        status = true;
                    }
                    if status
                        && self.alpm_helper.to_remove(&String::from(app["pkg"].as_str().unwrap()))
                    {
                        status = false;
                    }

                    let mut alpm_packages_vec = vec![String::from(app["pkg"].as_str().unwrap())];
                    {
                        let alpm_packages_temp = app["extra"].as_array().unwrap();
                        for alpm_package in alpm_packages_temp {
                            alpm_packages_vec.push(alpm_package.as_str().unwrap().to_owned());
                        }
                    }

                    let alpm_packages = alpm_packages_vec.join(" ");

                    self.app_store.insert_with_values(Some(&index), None, &[
                        (GROUP, &None::<String>),
                        (ICON, &String::from(app["icon"].as_str().unwrap())),
                        (APPLICATION, &String::from(app["name"].as_str().unwrap())),
                        (DESCRIPTION, &String::from(app["description"].as_str().unwrap())),
                        (ACTIVE, &status),
                        (PACKAGE, &alpm_packages),
                        (INSTALLED, &status),
                    ]);
                }
            }
        }

        store_size
    }

    pub fn reload_app_data(&mut self, refresh: bool) {
        self.alpm_helper.clear();
        self.app_store.clear();

        if refresh {
            self.alpm_handle = new_alpm().unwrap();
            self.group_store = load_groups_data(&self.groups);
        }
        self.load_app_data();
        self.tree_view.set_model(Some(&self.app_store));
        self.update_system_btn.set_sensitive(!self.alpm_helper.is_empty());
    }

    fn create_view_tree(&mut self) -> usize {
        // setup list store model
        let app_store_size = self.load_app_data();

        // create a tree view with the model store
        self.tree_view = gtk::TreeView::with_model(&self.app_store);
        self.tree_view.set_activate_on_single_click(true);
        self.tree_view.set_has_tooltip(true);
        self.tree_view.connect_query_tooltip(on_query_tooltip_tree_view);
        self.tree_view.connect_button_press_event(on_button_press_event_tree_view);

        // column model: icon
        let icon = gtk::CellRendererPixbuf::new();
        let icon_column = create_column("", &icon, "icon_name", ICON);
        self.tree_view.append_column(&icon_column);

        // column model: group name column
        // let group_cell_renderer = gtk::CellRendererText::new();
        // let group_column = create_column("Group", &group_cell_renderer, "text", APPLICATION);
        // tree_view.append_column(&group_column);
        // group_column
        //    .set_cell_data_func(&group_cell_renderer,
        // Some(Box::new(treeview_cell_app_data_function)));

        // column model: app name column
        let app_cell_renderer = gtk::CellRendererText::new();
        let app_column = create_column("Application", &app_cell_renderer, "text", APPLICATION);
        // app_column.set_resizable(false);
        app_column.set_cell_data_func(
            &app_cell_renderer,
            Some(Box::new(treeview_cell_app_data_function)),
        );
        self.tree_view.append_column(&app_column);

        // column model: description column
        let desc_renderer = gtk::CellRendererText::new();
        desc_renderer.set_wrap_mode(gtk::pango::WrapMode::Word);
        desc_renderer.set_wrap_width(290);
        let desc_column = create_column("Description", &desc_renderer, "text", DESCRIPTION);
        desc_column.set_resizable(false);
        self.tree_view.append_column(&desc_column);

        // column model: install column
        let install_renderer = gtk::CellRendererToggle::new();
        install_renderer.connect_toggled(on_app_toggle);
        let install_column = create_column("Install/Remove", &install_renderer, "active", ACTIVE);
        install_column.set_cell_data_func(
            &install_renderer,
            Some(Box::new(treeview_cell_check_data_function)),
        );

        install_column.set_resizable(false);
        install_column.set_max_width(40);
        install_column.set_fixed_width(40);
        self.tree_view.append_column(&install_column);

        app_store_size
    }

    pub fn get_page(&self) -> &gtk::Box {
        &self.app_browser_box
    }

    fn create_page(&mut self) {
        // create view and app store
        let app_store_size = self.create_view_tree();
        // create a scrollable window
        let app_window = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        app_window.set_vexpand(true);
        app_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        // add window to tree view
        app_window.add(&self.tree_view);

        // setup grid
        let grid_inter = gtk::Grid::new();
        grid_inter.set_column_homogeneous(true);
        grid_inter.set_row_homogeneous(true);
        // add grid to app browser
        self.app_browser_box.add(&grid_inter);
        grid_inter.attach(&app_window, 0, 0, 5, app_store_size as i32);
    }

    pub fn get_alpm_handle(&self) -> &alpm::Alpm {
        &self.alpm_handle
    }
}

fn treeview_cell_app_data_function(
    _column: &gtk::TreeViewColumn,
    renderer_cell: &gtk::CellRenderer,
    model: &gtk::TreeModel,
    iter_a: &gtk::TreeIter,
) {
    let value_gobj = model.value(iter_a, INSTALLED as i32).get::<i32>();
    match value_gobj {
        Ok(1) | Ok(0) => renderer_cell.set_width(280),
        _ => (),
    };
}

fn treeview_cell_check_data_function(
    _column: &gtk::TreeViewColumn,
    renderer_cell: &gtk::CellRenderer,
    model: &gtk::TreeModel,
    iter_a: &gtk::TreeIter,
) {
    // hide checkbox for groups
    let value = model.value(iter_a, INSTALLED as i32).get::<i32>().unwrap();
    renderer_cell.set_visible(value != -1);
}

fn on_reload_clicked(_button: &gtk::Button) {
    let app_browser = unsafe { &mut G_APP_BROWSER.lock().unwrap() };
    app_browser.reload_app_data(false);
}

fn on_group_filter_changed(combo: &gtk::ComboBox) {
    let app_browser = unsafe { &mut G_APP_BROWSER.lock().unwrap() };
    if let Some(tree_iter) = combo.active_iter() {
        let model = combo.model().unwrap();
        let group_gobj = model.value(&tree_iter, 0);
        let group = group_gobj.get::<&str>().unwrap();
        app_browser.group_tofilter = String::from(group);
        app_browser.app_store.clear();
        app_browser.load_app_data();
        app_browser.tree_view.set_model(Some(&app_browser.app_store));
        if group != "*" {
            app_browser.tree_view.expand_all();
        }
    }
}

fn on_advanced_clicked(button: &gtk::ToggleButton) {
    let app_browser = unsafe { &mut G_APP_BROWSER.lock().unwrap() };
    let is_active = button.is_active();
    app_browser.filter = is_active;
    app_browser.reload_app_data(false);
}

fn on_query_tooltip_tree_view(
    treeview: &gtk::TreeView,
    x_f: i32,
    y_f: i32,
    keyboard_tip: bool,
    tooltip: &gtk::Tooltip,
) -> bool {
    let mut x = x_f;
    let mut y = y_f;
    let tooltip_context = treeview.tooltip_context(&mut x, &mut y, keyboard_tip);
    if let Some((model_tmp, path, iter_a)) = tooltip_context {
        let model = model_tmp.unwrap();
        let value = model.value(&iter_a, INSTALLED as i32).get::<i32>().unwrap();
        if value == 1 {
            let mut msg = String::from("Installed");
            let active = model.value(&iter_a, ACTIVE as i32).get::<i32>().unwrap();
            if active == 0 {
                msg.push_str(" , to remove");
            }
            tooltip.set_markup(Some(msg.as_str()));
            treeview.set_tooltip_row(tooltip, &path);
            return true;
        }
    }
    false
}

fn on_button_press_event_tree_view(
    treeview: &gtk::TreeView,
    event_btn: &gdk::EventButton,
) -> gtk::glib::signal::Inhibit {
    if event_btn.button() == 1 && event_btn.event_type() == gdk::EventType::DoubleButtonPress {
        if let Some(coords) = event_btn.coords() {
            let (x, y) = coords;
            let path_info = treeview.path_at_pos(x as i32, y as i32);
            if path_info.is_none() {
                return gtk::glib::signal::Inhibit(true);
            }

            let (path, ..) = path_info.unwrap();
            let app_browser = unsafe { &mut G_APP_BROWSER.lock().unwrap() };
            let app_store = &app_browser.app_store;
            let iter_a = app_store.iter(path.as_ref().unwrap()).unwrap();
            let value_gobj = app_store.value(&iter_a, PACKAGE as i32);

            if value_gobj.get::<&str>().is_err() {
                if treeview.row_expanded(path.as_ref().unwrap()) {
                    treeview.collapse_row(&path.unwrap());
                } else {
                    treeview.expand_to_path(&path.unwrap());
                }
            }
        }
    }

    gtk::glib::signal::Inhibit(false)
}

fn on_app_toggle(_cell: &gtk::CellRendererToggle, path: gtk::TreePath) {
    let app_browser = unsafe { &mut G_APP_BROWSER.lock().unwrap() };
    let app_store = &app_browser.app_store;
    let iter_a = app_store.iter(&path).unwrap();
    let value_gobj = app_store.value(&iter_a, PACKAGE as i32);

    // a group has no package attached and we don't install groups
    if value_gobj.get::<&str>().is_ok() {
        let toggle_a = app_store.value(&iter_a, ACTIVE as i32).get::<i32>().unwrap() == 1;
        app_store.set(&iter_a, &[(ACTIVE, &!toggle_a)]);

        let alpm_handle = app_browser.get_alpm_handle();
        let update_system_button = app_browser.update_system_btn.clone();
        let localdb = alpm_handle.localdb();
        let alpm_packages = app_store.value(&iter_a, PACKAGE as i32).get::<String>().unwrap();
        let alpm_packages_vec = alpm_packages.split(' ').map(String::from).collect::<Vec<String>>();

        let pkg = alpm_packages_vec.first().unwrap();

        let installed = localdb.pkg(pkg.as_bytes()).is_ok();
        // update lists
        app_browser.alpm_helper.set_package(&alpm_packages, !toggle_a, installed);
        update_system_button.set_sensitive(!app_browser.alpm_helper.is_empty());
    }
}

fn on_update_system_clicked(_: &gtk::Button) {
    let app_browser = unsafe { &mut G_APP_BROWSER.lock().unwrap() };
    if app_browser.alpm_helper.do_update() != AlpmHelperResult::Nothing {
        // reload json for view new apps installed
        app_browser.reload_app_data(true);
    }
}

fn load_groups_data(groups: &serde_json::Value) -> gtk::ListStore {
    // not use data set for the moment
    let store = gtk::ListStore::new(&[String::static_type()]);
    store.set(&store.append(), &[(0, &String::from("*"))]);

    for group in groups.as_array().unwrap() {
        let g_name = String::from(group["name"].as_str().unwrap());
        store.set(&store.append(), &[(0, &g_name)]);
    }

    store
}

fn create_column(
    title: &str,
    cell: &impl IsA<gtk::CellRenderer>,
    attr: &str,
    val: u32,
) -> gtk::TreeViewColumn {
    let column = gtk::TreeViewColumn::new();
    column.set_title(title);
    column.pack_start(cell, true);
    column.add_attribute(cell, attr, val as i32);

    column
}
