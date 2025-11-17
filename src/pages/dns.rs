use crate::ui::UI;
use crate::{actions, create_gtk_button, dns, fl, utils};

use gtk::prelude::*;

use gtk::{glib, Builder};

enum DnsSelection {
    Predefined(usize),
    Custom { ipv4: String, ipv6: String },
}

fn selection_index_for_connection(conn_name: &str) -> DnsSelection {
    if let Some((ipv4_dns, ipv6_dns)) = actions::get_dns_for_connection(conn_name) {
        for (key_index, (_name, (ipv4_map, ipv6_map))) in dns::G_DNS_SERVERS.entries().enumerate() {
            if (!ipv4_dns.is_empty() && &ipv4_dns == ipv4_map)
                || (!ipv6_dns.is_empty() && &ipv6_dns == ipv6_map)
            {
                return DnsSelection::Predefined(key_index);
            }
        }

        // fallback to custom
        if !ipv4_dns.is_empty() || !ipv6_dns.is_empty() {
            return DnsSelection::Custom { ipv4: ipv4_dns, ipv6: ipv6_dns };
        }
    }

    // fallback to Cloudflare
    DnsSelection::Predefined(dns::G_DNS_SERVERS.get_index("Cloudflare").unwrap())
}

fn create_connections_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let connection_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let dnsservers_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let custom_ipv4_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let custom_ipv6_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
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

    let ipv4_label = gtk::Label::new(None);
    ipv4_label.set_justify(gtk::Justification::Left);
    ipv4_label.set_text(&fl!("custom-ipv4-label"));
    ipv4_label.set_widget_name("custom-ipv4-label");
    let ipv6_label = gtk::Label::new(None);
    ipv6_label.set_justify(gtk::Justification::Left);
    ipv6_label.set_text(&fl!("custom-ipv6-label"));
    ipv6_label.set_widget_name("custom-ipv6-label");

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
        for dns_server in dns::G_DNS_SERVERS.keys() {
            store.set(&store.append(), &[(0, dns_server)]);
        }
        store.set(&store.append(), &[(0, &fl!("custom-dns-server"))]);
        utils::create_combo_with_model(&store)
    };

    let ipv4_entry = gtk::Entry::new();
    ipv4_entry.set_placeholder_text(Some("8.8.8.8,8.8.4.4"));
    let ipv6_entry = gtk::Entry::new();
    ipv6_entry.set_placeholder_text(Some("2001:4860:4860::8888"));

    combo_conn.set_widget_name("connections_combo");
    combo_servers.set_widget_name("servers_combo");
    
    custom_ipv4_box.set_visible(false);
    custom_ipv6_box.set_visible(false);

    let update_ui_for_connection = glib::clone!(
        @weak combo_servers,
        @weak ipv4_entry,
        @weak ipv6_entry,
        @weak custom_ipv4_box,
        @weak custom_ipv6_box
        => move |conn_name: &str| {
        let selection = selection_index_for_connection(conn_name);
        match selection {
            DnsSelection::Predefined(index) => {
                combo_servers.set_active(Some(index as u32));
                ipv4_entry.set_text("");
                ipv6_entry.set_text("");
                custom_ipv4_box.set_visible(false);
                custom_ipv6_box.set_visible(false);
            }
            DnsSelection::Custom { ipv4, ipv6 } => {
                // assuming custom is the last field
                combo_servers.set_active(Some(dns::G_DNS_SERVERS.len() as u32));
                ipv4_entry.set_text(&ipv4);
                ipv6_entry.set_text(&ipv6);
                custom_ipv4_box.set_visible(true);
                custom_ipv6_box.set_visible(true);
            }
        }
    });

    // preset the current active connection
    if let Some(active_conn_name) = actions::get_active_connection_name() {
        let model = combo_conn.model().unwrap();
        if let Some(iter) = model.iter_first() {
            loop {
                if model.value(&iter, 0).get::<String>().unwrap() == active_conn_name {
                    combo_conn.set_active_iter(Some(&iter));

                    update_ui_for_connection(&active_conn_name);
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

        update_ui_for_connection(&conn_name);
    });

    // show custom dns fields only when selected
    let custom_servers_label = fl!("custom-dns-server");
    let custom_servers_label_clone = custom_servers_label.clone();
    combo_servers.connect_changed(
        glib::clone!(@weak custom_ipv4_box, @weak custom_ipv6_box => move |combo| {
            let conn_name = if let Some(tree_iter) = combo.active_iter() {
                let model = combo.model().unwrap();
                model.value(&tree_iter, 0).get::<String>().unwrap()
            } else {
                // use empty string which will trigger fallback
                "".to_owned()
            };

            let is_custom = conn_name == custom_servers_label;
            custom_ipv4_box.set_visible(is_custom);
            custom_ipv6_box.set_visible(is_custom);
        }),
    );
    ipv4_entry.set_visible(false);
    ipv6_entry.set_visible(false);

    // Create context channel.
    let (dialog_tx, dialog_rx) = glib::MainContext::channel(glib::Priority::default());

    // Connect signals.
    let combo_conn_clone = combo_conn.clone();
    let combo_serv_clone = combo_servers.clone();
    apply_btn.connect_clicked(glib::clone!(
        @weak combo_conn,
        @weak combo_servers,
        @weak ipv4_entry,
        @weak ipv6_entry,
        @strong custom_servers_label_clone,
        @strong dialog_tx
        => move |_| {
        let dialog_tx_clone = dialog_tx.clone();
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

        let (ipv4_addr, ipv6_addr) = if server_name != custom_servers_label_clone {
            let (ipv4_addr, ipv6_addr) = dns::G_DNS_SERVERS.get(&server_name).unwrap();
            (ipv4_addr.to_string(), ipv6_addr.to_string())
        } else {
            (ipv4_entry.text().to_string(), ipv6_entry.text().to_string())
        };

        std::thread::spawn(move || {
            actions::change_dns_server(&conn_name, &ipv4_addr, &ipv6_addr, dialog_tx_clone);
        });
    }));

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
        let ui_comp = crate::gui::GUI::new(widget_window);

        ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
        glib::ControlFlow::Continue
    });

    topbox.pack_start(&label, true, false, 1);
    connection_box.pack_start(&connections_label, true, true, 2);
    connection_box.pack_end(&combo_conn, true, true, 2);
    dnsservers_box.pack_start(&servers_label, true, true, 2);
    dnsservers_box.pack_end(&combo_servers, true, true, 2);

    custom_ipv4_box.pack_start(&ipv4_label, false, false, 2);
    custom_ipv4_box.pack_start(&ipv4_entry, true, true, 2);
    custom_ipv6_box.pack_start(&ipv6_label, false, false, 2);
    custom_ipv6_box.pack_start(&ipv6_entry, true, true, 2);

    button_box.pack_start(&reset_btn, true, true, 2);
    button_box.pack_end(&apply_btn, true, true, 2);
    connection_box.set_halign(gtk::Align::Fill);
    dnsservers_box.set_halign(gtk::Align::Fill);
    button_box.set_halign(gtk::Align::Fill);
    topbox.pack_start(&connection_box, true, true, 5);
    topbox.pack_start(&dnsservers_box, true, true, 5);
    topbox.pack_start(&custom_ipv4_box, true, true, 5);
    topbox.pack_start(&custom_ipv6_box, true, true, 5);
    topbox.pack_start(&button_box, true, true, 5);

    topbox.set_hexpand(true);
    topbox
}

pub fn create_connections_page(builder: &Builder) {
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
