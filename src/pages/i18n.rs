use crate::fl;

use gtk::prelude::*;

use gtk::Builder;

fn update_translation_apps_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Some(section_label) = section_box_element.downcast_ref::<gtk::Label>() {
            section_label.set_text(&fl!("applications"));
        }
    }
}

fn update_translation_button_section(
    section_box: &gtk::Box,
    section_title: &str,
    button_label: impl Fn(&str) -> String,
) {
    for section_box_element in section_box.children() {
        if let Some(button_box) = section_box_element.downcast_ref::<gtk::Box>() {
            for button_box_widget in button_box.children() {
                let box_element_btn = button_box_widget.downcast::<gtk::Button>().unwrap();
                let widget_name = box_element_btn.widget_name();
                box_element_btn.set_label(&button_label(&widget_name));
            }
        } else if let Some(section_label) = section_box_element.downcast_ref::<gtk::Label>() {
            section_label.set_text(section_title);
        }
    }
}

fn update_translation_connections_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(object_box) = section_box_element.clone().downcast::<gtk::Box>() {
            match object_box.widget_name().as_str() {
                "dns-connection-box" | "dns-servers-box" | "dns-button-box" | "dns-latency-box"
                | "dns-dot-box" => {},
                _ => continue,
            }
            for object_box_widget in object_box.children() {
                let widget_name = object_box_widget.widget_name();
                if let Ok(box_element_check) =
                    object_box_widget.clone().downcast::<gtk::CheckButton>()
                {
                    let translated_text = match widget_name.as_str() {
                        "enable-dot" => fl!("enable-encrypted-dns", protocol = "TLS", abbr = "DoT"),
                        "enable-doh" => {
                            fl!("enable-encrypted-dns", protocol = "HTTPS", abbr = "DoH")
                        },
                        "enable-doq" => {
                            fl!("enable-encrypted-dns", protocol = "QUIC", abbr = "DoQ")
                        },
                        _ => crate::localization::get_locale_text(&widget_name),
                    };
                    box_element_check.set_label(&translated_text);
                } else if let Ok(box_element_btn) =
                    object_box_widget.clone().downcast::<gtk::Button>()
                {
                    let translated_text = crate::localization::get_locale_text(&widget_name);
                    box_element_btn.set_label(&translated_text);
                } else if let Ok(box_element_label) = object_box_widget.downcast::<gtk::Label>() {
                    let translated_text = crate::localization::get_locale_text(&widget_name);
                    box_element_label.set_text(&translated_text);
                }
            }
        } else if let Ok(section_label) = section_box_element.downcast::<gtk::Label>() {
            section_label.set_text(&fl!("dns-settings"));
        }
    }
}

pub fn update_translations(builder: &Builder) {
    // Update buttons
    let tweakbrowser_btn: gtk::Button = builder.object("tweaksBrowser").unwrap();
    tweakbrowser_btn.set_label(&fl!("tweaksbrowser-label"));
    tweakbrowser_btn.set_tooltip_text(Some(&fl!("tweaksbrowser-label")));

    let appbrowser_btn: gtk::Button = builder.object("appBrowser").unwrap();
    appbrowser_btn.set_label(&fl!("appbrowser-label"));
    appbrowser_btn.set_tooltip_text(Some(&fl!("appbrowser-label")));

    let troubleshooting_btn: gtk::Button = builder.object("troubleshooting").unwrap();
    troubleshooting_btn.set_label(&fl!("troubleshooting-label"));
    troubleshooting_btn.set_tooltip_text(Some(&fl!("troubleshooting-label")));

    let stack: gtk::Stack = builder.object("stack").unwrap();
    if let Some(content) = super::page_content_box(&stack, "tweaksBrowserpage") {
        for section_widget in content.children() {
            let section_box = section_widget.downcast::<gtk::Box>().unwrap();

            match section_box.widget_name().as_str() {
                "tweaksBrowserpage_options" => {
                    update_translation_button_section(&section_box, &fl!("tweaks"), |msgid| {
                        fl!("tweak-enabled-title", tweak = msgid)
                    });
                },
                "tweaksBrowserpage_fixes" => {
                    update_translation_button_section(
                        &section_box,
                        &fl!("fixes"),
                        crate::localization::get_locale_text,
                    );
                },
                "tweaksBrowserpage_apps" => {
                    update_translation_apps_section(&section_box);
                },
                _ => panic!("Unknown widget!"),
            }
        }
    }
    if let Some(content) = super::page_content_box(&stack, "dnsConnectionsBrowserpage") {
        for section_widget in content.children() {
            let section_box = section_widget.downcast::<gtk::Box>().unwrap();
            update_translation_connections_section(&section_box);
        }
    }
    if let Some(content) = super::page_content_box(&stack, "troubleshootingpage") {
        for section_widget in content.children() {
            let section_box = section_widget.downcast::<gtk::Box>().unwrap();
            update_translation_button_section(
                &section_box,
                &fl!("troubleshooting"),
                crate::localization::get_locale_text,
            );
        }
    }
}
