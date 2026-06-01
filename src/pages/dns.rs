use crate::ui::{Action, DialogMessage, MessageType, UI};
use crate::{actions, create_gtk_button, dns, fl, utils};

use gtk::prelude::*;

use gtk::{Builder, glib};

/// Returns true if `s` contains only valid DNS address characters (hex digits, dots, colons,
/// commas).
fn is_valid_dns_input(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit() || matches!(c, '.' | ':' | ','))
}

fn selection_index_for_connection(conn_name: &str) -> usize {
    // If blocky is active (DoH/DoQ mode) AND this connection points to blocky (127.0.0.1),
    // read the blocky config to find which preset server is in use.
    if actions::is_blocky_active() {
        let points_to_blocky = actions::get_dns_for_connection(conn_name)
            .is_some_and(|info| info.ipv4.contains("127.0.0.1") || info.ipv6.contains("::1"));
        if points_to_blocky
            && let Some((mode, upstream)) = dns::read_active_blocky()
        {
            return dns::find_server_by_blocky_upstream(&upstream, mode)
                .unwrap_or(dns::G_DNS_SERVERS.len());
        }
    }

    if let Some(dns_info) = actions::get_dns_for_connection(conn_name) {
        for (key_index, (_name, (ipv4_map, ipv6_map, _dot))) in
            dns::G_DNS_SERVERS.entries().enumerate()
        {
            if (!dns_info.ipv4.is_empty() && &dns_info.ipv4 == ipv4_map)
                || (!dns_info.ipv6.is_empty() && &dns_info.ipv6 == ipv6_map)
            {
                return key_index;
            }
        }
        // DNS is set but doesn't match any preset — custom server
        return dns::G_DNS_SERVERS.len();
    }

    // No DNS configured — using DHCP (automatic)
    usize::MAX
}

/// Returns whether the server at `index` supports `DoT`.
fn server_supports_dot(index: usize) -> bool {
    dns::G_DNS_SERVERS.entries().nth(index).is_some_and(|(_, (_, _, dot))| dot.is_some())
}

/// Returns whether the server at `index` supports blocky encrypted DNS.
fn server_supports_blocky(index: usize, mode: dns::BlockyMode) -> bool {
    dns::G_DNS_SERVERS
        .entries()
        .nth(index)
        .is_some_and(|(name, _)| dns::get_blocky_upstream(name, mode).is_some())
}

fn disable_protocol_checks(
    dot_check: &gtk::CheckButton,
    doh_check: &gtk::CheckButton,
    doq_check: &gtk::CheckButton,
) {
    for check in [dot_check, doh_check, doq_check] {
        check.set_sensitive(false);
        check.set_active(false);
    }
}

fn fill_custom_blocky_fields(
    upstream: &str,
    mode: dns::BlockyMode,
    custom_doh_entry: &gtk::Entry,
    custom_doq_entry: &gtk::Entry,
    doh_check: &gtk::CheckButton,
    doq_check: &gtk::CheckButton,
) {
    match mode {
        dns::BlockyMode::Doh => {
            custom_doh_entry.set_text(upstream);
            doh_check.set_active(true);
            doq_check.set_active(false);
        },
        dns::BlockyMode::Doq => {
            custom_doq_entry.set_text(upstream);
            doq_check.set_active(true);
            doh_check.set_active(false);
        },
    }
}

fn set_preset_blocky_check(
    check: &gtk::CheckButton,
    index: usize,
    mode: dns::BlockyMode,
    blocky_active: bool,
    active_mode: Option<dns::BlockyMode>,
) {
    let supported = server_supports_blocky(index, mode);
    check.set_sensitive(supported);
    check.set_active(blocky_active && supported && active_mode == Some(mode));
}

/// Returns (region, homepage) for the server at `index`.
fn server_info_at(index: usize) -> Option<(&'static str, &'static str)> {
    let (name, _) = dns::G_DNS_SERVERS.entries().nth(index)?;
    let info = dns::G_DNS_SERVER_INFO.get(name)?;
    Some((info.region, info.homepage))
}

/// Update the info label markup for the selected server index.
fn update_server_info_label(info_label: &gtk::Label, index: usize) {
    if let Some((region, homepage)) = server_info_at(index) {
        info_label.set_markup(&format!(
            "<small>{region} - <a href=\"{homepage}\">{homepage}</a></small>"
        ));
        info_label.set_visible(true);
    } else {
        info_label.set_visible(false);
    }
}

fn create_connections_section() -> gtk::Box {
    let topbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let connection_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let dnsservers_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let dot_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
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
    let apply_btn = create_gtk_button!("apply");
    let reset_btn = create_gtk_button!("reset");

    let combo_conn = {
        let store = gtk::ListStore::new(&[String::static_type()]);
        let nm_connections = actions::get_nm_connections();
        for nm_connection in &nm_connections {
            store.set(&store.append(), &[(0, nm_connection)]);
        }
        utils::create_combo_with_model(&store)
    };
    let combo_servers = {
        let store = gtk::ListStore::new(&[String::static_type()]);
        for dns_server in dns::G_DNS_SERVERS.keys() {
            store.set(&store.append(), &[(0, dns_server)]);
        }
        let custom_label = fl!("custom-dns");
        store.set(&store.append(), &[(0, &custom_label)]);
        let dhcp_label = fl!("dhcp-automatic");
        store.set(&store.append(), &[(0, &dhcp_label)]);
        utils::create_combo_with_model(&store)
    };

    combo_conn.set_widget_name("connections_combo");
    combo_servers.set_widget_name("servers_combo");

    // DoT (DNS over TLS) toggle
    let dot_check = gtk::CheckButton::with_label(&fl!("enable-dot"));
    dot_check.set_tooltip_text(Some(&fl!("dot-tooltip")));
    dot_check.set_widget_name("enable-dot");

    // DoH (DNS over HTTPS) toggle — uses blocky local proxy
    let doh_check = gtk::CheckButton::with_label(&fl!("enable-doh"));
    doh_check.set_tooltip_text(Some(&fl!("doh-tooltip")));
    doh_check.set_widget_name("enable-doh");

    // DoQ (DNS over QUIC) toggle — uses blocky local proxy
    let doq_check = gtk::CheckButton::with_label(&fl!("enable-doq"));
    doq_check.set_tooltip_text(Some(&fl!("doq-tooltip")));
    doq_check.set_widget_name("enable-doq");

    // DoT, DoH, and DoQ are mutually exclusive
    let dot_check_excl = dot_check.clone();
    let doh_check_excl = doh_check.clone();
    let doq_check_excl = doq_check.clone();
    dot_check.connect_toggled(glib::clone!(@weak doh_check_excl, @weak doq_check_excl => move |check| {
        if check.is_active() {
            doh_check_excl.set_active(false);
            doq_check_excl.set_active(false);
        }
    }));
    doh_check.connect_toggled(glib::clone!(@weak dot_check_excl, @weak doq_check_excl => move |check| {
        if check.is_active() {
            dot_check_excl.set_active(false);
            doq_check_excl.set_active(false);
        }
    }));
    doq_check.connect_toggled(glib::clone!(@weak dot_check_excl, @weak doh_check_excl => move |check| {
        if check.is_active() {
            dot_check_excl.set_active(false);
            doh_check_excl.set_active(false);
        }
    }));

    // Server info label (region + homepage link)
    let info_label = gtk::Label::new(None);
    info_label.set_use_markup(true);
    info_label.set_xalign(0.5);
    info_label.set_widget_name("server-info");

    // Custom DNS input fields (shown when "Custom" is selected)
    let custom_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
    let custom_ipv4_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let custom_ipv6_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let custom_dot_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let custom_doh_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let custom_doq_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);

    let custom_ipv4_label = gtk::Label::new(None);
    custom_ipv4_label.set_text(&fl!("custom-dns-ipv4"));
    let custom_ipv4_entry = gtk::Entry::new();
    custom_ipv4_entry.set_placeholder_text(Some("e.g. 1.1.1.1,1.0.0.1"));
    custom_ipv4_entry.set_widget_name("custom-dns-ipv4");

    let custom_ipv6_label = gtk::Label::new(None);
    custom_ipv6_label.set_text(&fl!("custom-dns-ipv6"));
    let custom_ipv6_entry = gtk::Entry::new();
    custom_ipv6_entry.set_placeholder_text(Some("e.g. 2606:4700:4700::1111"));
    custom_ipv6_entry.set_widget_name("custom-dns-ipv6");

    let custom_dot_label = gtk::Label::new(None);
    custom_dot_label.set_text(&fl!("custom-dns-dot-hostname"));
    let custom_dot_entry = gtk::Entry::new();
    custom_dot_entry.set_placeholder_text(Some("e.g. dns.example.com"));
    custom_dot_entry.set_widget_name("custom-dns-dot-hostname");

    let custom_doh_label = gtk::Label::new(None);
    custom_doh_label.set_text(&fl!("custom-dns-doh-url"));
    let custom_doh_entry = gtk::Entry::new();
    custom_doh_entry.set_placeholder_text(Some("e.g. https://dns.example.com/dns-query"));
    custom_doh_entry.set_widget_name("custom-dns-doh-url");

    let custom_doq_label = gtk::Label::new(None);
    custom_doq_label.set_text(&fl!("custom-dns-doq-endpoint"));
    let custom_doq_entry = gtk::Entry::new();
    custom_doq_entry.set_placeholder_text(Some("e.g. quic:dns.example.com"));
    custom_doq_entry.set_widget_name("custom-dns-doq-endpoint");

    custom_ipv4_box.pack_start(&custom_ipv4_label, true, true, 2);
    custom_ipv4_box.pack_end(&custom_ipv4_entry, true, true, 2);
    custom_ipv6_box.pack_start(&custom_ipv6_label, true, true, 2);
    custom_ipv6_box.pack_end(&custom_ipv6_entry, true, true, 2);
    custom_dot_box.pack_start(&custom_dot_label, true, true, 2);
    custom_dot_box.pack_end(&custom_dot_entry, true, true, 2);
    custom_doh_box.pack_start(&custom_doh_label, true, true, 2);
    custom_doh_box.pack_end(&custom_doh_entry, true, true, 2);
    custom_doq_box.pack_start(&custom_doq_label, true, true, 2);
    custom_doq_box.pack_end(&custom_doq_entry, true, true, 2);

    custom_box.pack_start(&custom_ipv4_box, false, false, 2);
    custom_box.pack_start(&custom_ipv6_box, false, false, 2);
    custom_box.pack_start(&custom_dot_box, false, false, 2);
    custom_box.pack_start(&custom_doh_box, false, false, 2);
    custom_box.pack_start(&custom_doq_box, false, false, 2);
    custom_box.set_widget_name("dns-custom-box");
    custom_box.set_no_show_all(true);
    custom_box.set_visible(false);

    // Latency test button and result label
    let latency_btn = gtk::Button::with_label(&fl!("test-latency"));
    latency_btn.set_tooltip_text(Some(&fl!("test-latency-tooltip")));
    latency_btn.set_widget_name("test-latency");
    let latency_label = gtk::Label::new(None);
    latency_label.set_widget_name("latency-result");

    // Best server button (auto-select lowest latency)
    let best_btn = gtk::Button::with_label(&fl!("best-server"));
    best_btn.set_tooltip_text(Some(&fl!("best-server-tooltip")));
    best_btn.set_widget_name("best-server");

    // preset the current active connection
    if let Some(active_conn_name) = actions::get_active_connection_name() {
        let model = combo_conn.model().unwrap();
        if let Some(iter) = utils::find_iter_in_model(&model, &active_conn_name) {
            combo_conn.set_active_iter(Some(&iter));

            let selected_dns_index = selection_index_for_connection(&active_conn_name);
            let dhcp_index = dns::G_DNS_SERVERS.len() + 1;
            let combo_index =
                if selected_dns_index == usize::MAX { dhcp_index } else { selected_dns_index };
            combo_servers.set_active(Some(combo_index as u32));

            if selected_dns_index == usize::MAX {
                disable_protocol_checks(&dot_check, &doh_check, &doq_check);
            } else if selected_dns_index == dns::G_DNS_SERVERS.len() {
                // Custom DNS — pre-fill entries with current values
                if actions::is_blocky_active() {
                    if let Some((mode, upstream)) = dns::read_active_blocky() {
                        fill_custom_blocky_fields(
                            &upstream,
                            mode,
                            &custom_doh_entry,
                            &custom_doq_entry,
                            &doh_check,
                            &doq_check,
                        );
                    }
                    let (ipv4, ipv6, dot_host) = dns::read_blocky_bootstrap();
                    if !ipv4.is_empty() {
                        custom_ipv4_entry.set_text(&ipv4);
                    }
                    if !ipv6.is_empty() {
                        custom_ipv6_entry.set_text(&ipv6);
                    }
                    if let Some(ref hostname) = dot_host {
                        custom_dot_entry.set_text(hostname);
                    }
                    dot_check.set_active(false);
                } else {
                    if let Some(dns_info) = actions::get_dns_for_connection(&active_conn_name) {
                        custom_ipv4_entry.set_text(&dns_info.ipv4);
                        custom_ipv6_entry.set_text(&dns_info.ipv6);
                        if let Some(ref hostname) = dns_info.dot_hostname {
                            custom_dot_entry.set_text(hostname);
                        }
                    }
                    let dot_enabled = actions::get_dot_for_connection(&active_conn_name);
                    dot_check.set_active(dot_enabled);
                }
                custom_box.foreach(gtk::prelude::WidgetExt::show_all);
                custom_box.show();
                dot_check.set_sensitive(true);
                doh_check.set_sensitive(true);
                doq_check.set_sensitive(true);
            } else {
                let blocky_active = actions::is_blocky_active();
                let active_mode =
                    if blocky_active { dns::read_active_blocky().map(|(mode, _)| mode) } else { None };

                let supports_dot = server_supports_dot(selected_dns_index);
                dot_check.set_sensitive(supports_dot);
                dot_check.set_active(supports_dot && !blocky_active);

                set_preset_blocky_check(
                    &doh_check,
                    selected_dns_index,
                    dns::BlockyMode::Doh,
                    blocky_active,
                    active_mode,
                );
                set_preset_blocky_check(
                    &doq_check,
                    selected_dns_index,
                    dns::BlockyMode::Doq,
                    blocky_active,
                    active_mode,
                );
            }
            update_server_info_label(&info_label, selected_dns_index);
        }
    }

    // Update DoT/DoH/DoQ checkboxes, info label, and custom fields when server selection changes
    let dot_check_clone = dot_check.clone();
    let doh_check_clone = doh_check.clone();
    let doq_check_clone = doq_check.clone();
    let info_label_clone = info_label.clone();
    let custom_box_vis = custom_box.clone();
    let best_btn_vis = best_btn.clone();
    combo_servers.connect_changed(move |combo| {
        if let Some(idx) = combo.active() {
            let is_custom = idx as usize == dns::G_DNS_SERVERS.len();
            let is_dhcp = idx as usize == dns::G_DNS_SERVERS.len() + 1;
            if is_custom {
                custom_box_vis.foreach(gtk::prelude::WidgetExt::show_all);
                custom_box_vis.show();
            } else {
                custom_box_vis.hide();
            }
            best_btn_vis.set_visible(!is_custom && !is_dhcp);
            if is_dhcp {
                disable_protocol_checks(&dot_check_clone, &doh_check_clone, &doq_check_clone);
                info_label_clone.set_visible(false);
            } else if is_custom {
                dot_check_clone.set_sensitive(true);
                doh_check_clone.set_sensitive(true);
                doh_check_clone.set_active(false);
                doq_check_clone.set_sensitive(true);
                doq_check_clone.set_active(false);
                info_label_clone.set_visible(false);
            } else {
                let supports_dot = server_supports_dot(idx as usize);
                dot_check_clone.set_sensitive(supports_dot);
                dot_check_clone.set_active(supports_dot);
                set_preset_blocky_check(
                    &doh_check_clone,
                    idx as usize,
                    dns::BlockyMode::Doh,
                    false,
                    None,
                );
                set_preset_blocky_check(
                    &doq_check_clone,
                    idx as usize,
                    dns::BlockyMode::Doq,
                    false,
                    None,
                );
                update_server_info_label(&info_label_clone, idx as usize);
            }
        }
    });

    // select used dns option value on connection change
    let combo_servers_clone = combo_servers.clone();
    let dot_check_clone2 = dot_check.clone();
    let custom_ipv4_entry_conn = custom_ipv4_entry.clone();
    let custom_ipv6_entry_conn = custom_ipv6_entry.clone();
    let custom_dot_entry_conn = custom_dot_entry.clone();
    let custom_doh_entry_conn = custom_doh_entry.clone();
    let custom_doq_entry_conn = custom_doq_entry.clone();
    let doh_check_conn = doh_check.clone();
    let doq_check_conn = doq_check.clone();
    combo_conn.connect_changed(move |combo| {
        // use empty string which will trigger fallback
        let conn_name: String = combo.active_text().map(Into::into).unwrap_or_default();

        let selected_dns_index = selection_index_for_connection(&conn_name);
        combo_servers_clone.set_active(Some(selected_dns_index as u32));

        if selected_dns_index == dns::G_DNS_SERVERS.len() {
            // Custom DNS — pre-fill entries from blocky config or NM
            if actions::is_blocky_active() {
                if let Some((mode, upstream)) = dns::read_active_blocky() {
                    fill_custom_blocky_fields(
                        &upstream,
                        mode,
                        &custom_doh_entry_conn,
                        &custom_doq_entry_conn,
                        &doh_check_conn,
                        &doq_check_conn,
                    );
                }
                let (ipv4, ipv6, dot_host) = dns::read_blocky_bootstrap();
                custom_ipv4_entry_conn.set_text(&ipv4);
                custom_ipv6_entry_conn.set_text(&ipv6);
                custom_dot_entry_conn.set_text(dot_host.as_deref().unwrap_or(""));
            } else if let Some(dns_info) = actions::get_dns_for_connection(&conn_name) {
                custom_ipv4_entry_conn.set_text(&dns_info.ipv4);
                custom_ipv6_entry_conn.set_text(&dns_info.ipv6);
                if let Some(ref hostname) = dns_info.dot_hostname {
                    custom_dot_entry_conn.set_text(hostname);
                } else {
                    custom_dot_entry_conn.set_text("");
                }
            }
            dot_check_clone2.set_sensitive(true);
        } else {
            let supports_dot = server_supports_dot(selected_dns_index);
            dot_check_clone2.set_sensitive(supports_dot);
            if !supports_dot {
                dot_check_clone2.set_active(false);
            }
        }
    });

    // Latency test button handler
    let combo_serv_latency = combo_servers.clone();
    let custom_ipv4_entry_latency = custom_ipv4_entry.clone();
    let latency_label_clone = latency_label.clone();
    let latency_btn_clone = latency_btn.clone();
    let (latency_tx, latency_rx) = async_channel::unbounded();
    latency_btn.connect_clicked(move |_| {
        let is_custom =
            combo_serv_latency.active().is_some_and(|idx| idx as usize == dns::G_DNS_SERVERS.len());
        let ipv4 = if is_custom {
            let text = custom_ipv4_entry_latency.text().trim().to_string();
            if text.is_empty() {
                return;
            }
            text
        } else {
            let server_name: String =
                combo_serv_latency.active_text().map(Into::into).unwrap_or_default();
            let Some(server_addr) = dns::G_DNS_SERVERS.get(&server_name) else { return };
            server_addr.0.to_string()
        };
        let tx = latency_tx.clone();
        latency_btn_clone.set_sensitive(false);
        latency_label_clone.set_text(&fl!("latency-testing"));
        std::thread::spawn(move || {
            let result = dns::measure_latency(&ipv4);
            let _ = tx.send_blocking(result);
        });
    });
    let latency_label_rx = latency_label.clone();
    let latency_btn_rx = latency_btn.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Ok(result) = latency_rx.recv().await {
            match result {
                Some(ms) => latency_label_rx.set_markup(&format!("<b>{ms} ms</b>")),
                None => latency_label_rx.set_text(&fl!("latency-timeout")),
            }
            latency_btn_rx.set_sensitive(true);
        }
    });

    // Best server button handler
    let combo_serv_best = combo_servers.clone();
    let best_btn_clone = best_btn.clone();
    let latency_label_best = latency_label.clone();
    let (best_tx, best_rx) = async_channel::unbounded::<Option<(&'static str, u128)>>();
    best_btn.connect_clicked(move |_| {
        let tx = best_tx.clone();
        best_btn_clone.set_sensitive(false);
        latency_label_best.set_text(&fl!("latency-testing"));
        std::thread::spawn(move || {
            let results = dns::measure_all_latencies();
            let best = results
                .iter()
                .find(|(n, ms)| ms.is_some() && !dns::is_filtering_server(n))
                .map(|&(name, ms)| (name, ms.unwrap()));
            let _ = tx.send_blocking(best);
        });
    });
    let combo_serv_best_rx = combo_serv_best.clone();
    let best_btn_rx = best_btn.clone();
    let latency_label_best_rx = latency_label.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Ok(result) = best_rx.recv().await {
            match result {
                Some((name, ms)) => {
                    let model = combo_serv_best_rx.model().unwrap();
                    if let Some(iter) = utils::find_iter_in_model(&model, name) {
                        combo_serv_best_rx.set_active_iter(Some(&iter));
                    }
                    latency_label_best_rx.set_markup(&format!("<b>{ms} ms</b>"));
                },
                None => {
                    latency_label_best_rx.set_text(&fl!("latency-no-result"));
                },
            }
            best_btn_rx.set_sensitive(true);
        }
    });

    // Create context channel.
    let (dialog_tx, dialog_rx) = async_channel::unbounded();

    // Connect signals.
    let dialog_tx_clone = dialog_tx.clone();
    let combo_conn_clone = combo_conn.clone();
    let combo_serv_clone = combo_servers.clone();
    let dot_check_clone3 = dot_check.clone();
    let custom_ipv4_entry_apply = custom_ipv4_entry.clone();
    let custom_ipv6_entry_apply = custom_ipv6_entry.clone();
    let custom_dot_entry_apply = custom_dot_entry.clone();
    let custom_doh_entry_apply = custom_doh_entry.clone();
    let custom_doq_entry_apply = custom_doq_entry.clone();
    let doh_check_clone3 = doh_check.clone();
    let doq_check_clone3 = doq_check.clone();
    apply_btn.connect_clicked(move |_| {
        let conn_name: String = combo_conn_clone.active_text().map(Into::into).unwrap_or_default();
        let is_custom =
            combo_serv_clone.active().is_some_and(|idx| idx as usize == dns::G_DNS_SERVERS.len());
        let is_dhcp = combo_serv_clone
            .active()
            .is_some_and(|idx| idx as usize == dns::G_DNS_SERVERS.len() + 1);

        // DHCP (automatic) selected — reset to defaults
        if is_dhcp {
            let dialog_tx_clone = dialog_tx_clone.clone();
            std::thread::spawn(move || {
                actions::reset_dns_server(&conn_name, dialog_tx_clone);
            });
            return;
        }

        let enable_dot = dot_check_clone3.is_active();
        let enable_doh = doh_check_clone3.is_active();
        let enable_doq = doq_check_clone3.is_active();

        let (ipv4, ipv6, dot_hostname) = if is_custom {
            let ipv4: String = custom_ipv4_entry_apply.text().trim().to_string();
            let ipv6: String = custom_ipv6_entry_apply.text().trim().to_string();
            let hostname: String = custom_dot_entry_apply.text().trim().to_string();
            let ipv4_valid = ipv4.is_empty() || is_valid_dns_input(&ipv4);
            let ipv6_valid = ipv6.is_empty() || is_valid_dns_input(&ipv6);
            if (ipv4.is_empty() && ipv6.is_empty()) || !ipv4_valid || !ipv6_valid {
                let _ = dialog_tx_clone.try_send(DialogMessage {
                    msg: fl!("custom-dns-invalid"),
                    msg_type: MessageType::Error,
                    action: Action::SetDnsServer,
                });
                return;
            }
            if !hostname.is_empty() && !dns::is_valid_dot_hostname(&hostname) {
                let _ = dialog_tx_clone.try_send(DialogMessage {
                    msg: fl!("custom-dns-invalid-hostname"),
                    msg_type: MessageType::Error,
                    action: Action::SetDnsServer,
                });
                return;
            }
            (ipv4, ipv6, hostname)
        } else {
            let server_name: String =
                combo_serv_clone.active_text().map(Into::into).unwrap_or_default();
            let server_addr = dns::G_DNS_SERVERS.get(&server_name).unwrap();
            let hostname = server_addr.2.unwrap_or("").to_string();
            (server_addr.0.to_string(), server_addr.1.to_string(), hostname)
        };

        let dialog_tx_clone = dialog_tx_clone.clone();
        if let Some(mode) = match (enable_doh, enable_doq) {
            (true, _) => Some(dns::BlockyMode::Doh),
            (_, true) => Some(dns::BlockyMode::Doq),
            _ => None,
        } {
            let (upstream, bootstrap_ipv4, bootstrap_ipv6, bootstrap_dot) = if is_custom {
                let custom_upstream = match mode {
                    dns::BlockyMode::Doh => custom_doh_entry_apply.text().trim().to_string(),
                    dns::BlockyMode::Doq => custom_doq_entry_apply.text().trim().to_string(),
                };
                let valid = match mode {
                    dns::BlockyMode::Doh => dns::is_valid_doh_url(&custom_upstream),
                    dns::BlockyMode::Doq => dns::is_valid_doq_endpoint(&custom_upstream),
                };
                if !valid {
                    let msg = match mode {
                        dns::BlockyMode::Doh => fl!("custom-dns-doh-url-required"),
                        dns::BlockyMode::Doq => fl!("custom-dns-doq-endpoint-required"),
                    };
                    let _ = dialog_tx_clone.try_send(DialogMessage {
                        msg,
                        msg_type: MessageType::Error,
                        action: Action::SetDnsServer,
                    });
                    return;
                }
                let dot_host =
                    if dot_hostname.is_empty() { None } else { Some(dot_hostname.clone()) };
                (custom_upstream, ipv4.clone(), ipv6.clone(), dot_host)
            } else {
                let server_name: String =
                    combo_serv_clone.active_text().map(Into::into).unwrap_or_default();
                let server_addr = dns::G_DNS_SERVERS.get(&server_name).unwrap();
                (
                    dns::get_blocky_upstream(&server_name, mode).unwrap_or("").to_string(),
                    server_addr.0.to_string(),
                    server_addr.1.to_string(),
                    server_addr.2.map(String::from),
                )
            };
            std::thread::spawn(move || {
                actions::change_dns_server_blocky(
                    crate::gui::run_command,
                    &conn_name,
                    mode,
                    &upstream,
                    &bootstrap_ipv4,
                    &bootstrap_ipv6,
                    bootstrap_dot.as_deref(),
                    dialog_tx_clone,
                );
            });
        } else {
            // Stop blocky if switching away from encrypted DNS (in background thread)
            std::thread::spawn(move || {
                actions::stop_blocky();
                actions::change_dns_server(
                    &conn_name,
                    &ipv4,
                    &ipv6,
                    enable_dot,
                    &dot_hostname,
                    dialog_tx_clone,
                );
            });
        }
    });
    let dialog_tx_clone = dialog_tx.clone();
    let combo_conn_clone = combo_conn.clone();
    let combo_serv_reset = combo_servers.clone();
    let dot_check_reset = dot_check.clone();
    let doh_check_reset = doh_check.clone();
    let doq_check_reset = doq_check.clone();
    reset_btn.connect_clicked(move |_| {
        let dialog_tx_clone = dialog_tx_clone.clone();
        let conn_name: String = combo_conn_clone.active_text().map(Into::into).unwrap_or_default();
        // Set combo to DHCP (automatic)
        let dhcp_index = (dns::G_DNS_SERVERS.len() + 1) as u32;
        combo_serv_reset.set_active(Some(dhcp_index));
        dot_check_reset.set_active(false);
        doh_check_reset.set_active(false);
        doq_check_reset.set_active(false);
        std::thread::spawn(move || {
            actions::reset_dns_server(&conn_name, dialog_tx_clone);
        });
    });

    // Setup receiver
    let apply_btn_clone = apply_btn.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = dialog_rx.recv().await {
            let widget_obj = &apply_btn_clone;
            let widget_window =
                utils::get_window_from_widget(widget_obj).expect("Failed to retrieve window");
            let ui_comp = crate::gui::Gui::new(widget_window);

            ui_comp.show_message(msg.msg_type, &msg.msg, msg.msg_type.to_string());
        }
    });

    topbox.pack_start(&label, true, false, 1);
    connection_box.pack_start(&connections_label, true, true, 2);
    connection_box.pack_end(&combo_conn, true, true, 2);
    connection_box.set_widget_name("dns-connection-box");
    dnsservers_box.pack_start(&servers_label, true, true, 2);
    dnsservers_box.pack_end(&combo_servers, true, true, 2);
    dnsservers_box.set_widget_name("dns-servers-box");
    let latency_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    latency_box.set_halign(gtk::Align::Center);
    latency_box.set_widget_name("dns-latency-box");
    latency_box.pack_start(&latency_btn, false, false, 2);
    latency_box.pack_start(&best_btn, false, false, 2);
    latency_box.pack_start(&latency_label, false, false, 2);
    dot_box.pack_start(&dot_check, false, false, 2);
    dot_box.pack_start(&doh_check, false, false, 2);
    dot_box.pack_start(&doq_check, false, false, 2);
    dot_box.set_halign(gtk::Align::Center);
    dot_box.set_widget_name("dns-dot-box");
    button_box.pack_start(&reset_btn, true, true, 2);
    button_box.pack_end(&apply_btn, true, true, 2);
    button_box.set_widget_name("dns-button-box");
    connection_box.set_halign(gtk::Align::Fill);
    dnsservers_box.set_halign(gtk::Align::Fill);
    button_box.set_halign(gtk::Align::Fill);
    topbox.pack_start(&connection_box, true, true, 5);
    topbox.pack_start(&dnsservers_box, true, true, 5);
    let info_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    info_box.set_halign(gtk::Align::Center);
    info_box.pack_start(&info_label, false, false, 0);
    topbox.pack_start(&info_box, false, false, 2);
    topbox.pack_start(&custom_box, false, false, 2);
    topbox.pack_start(&latency_box, false, false, 2);
    topbox.pack_start(&dot_box, true, true, 5);
    topbox.pack_start(&button_box, true, true, 5);

    // DNS check link
    let check_label = gtk::Label::new(None);
    check_label.set_use_markup(true);
    check_label.set_markup(&format!(
        "<small>{} <a href=\"https://dnscheck.tools\">dnscheck.tools</a></small>",
        fl!("dns-check-hint")
    ));
    check_label.set_justify(gtk::Justification::Center);
    let check_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    check_box.set_halign(gtk::Align::Center);
    check_box.pack_start(&check_label, false, false, 0);
    topbox.pack_start(&check_box, false, false, 5);

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
