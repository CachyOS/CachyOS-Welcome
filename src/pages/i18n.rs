use crate::fl;

use gtk::prelude::*;

use gtk::Builder;

fn update_translation_apps_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(section_label) = section_box_element.clone().downcast::<gtk::Label>() {
            section_label.set_text(&fl!("applications"));
        }
    }
}

fn update_translation_fixes_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(button_box) = section_box_element.clone().downcast::<gtk::Box>() {
            for button_box_widget in button_box.children() {
                let box_element_btn = button_box_widget.downcast::<gtk::Button>().unwrap();
                let widget_name = box_element_btn.widget_name();
                let translated_text = crate::localization::get_locale_text(&widget_name);
                box_element_btn.set_label(&translated_text);
            }
        } else if let Ok(section_label) = section_box_element.downcast::<gtk::Label>() {
            section_label.set_text(&fl!("fixes"));
        }
    }
}

fn update_translation_connections_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(object_box) = section_box_element.clone().downcast::<gtk::Box>() {
            match object_box.widget_name().as_str() {
                "dns-connection-box" | "dns-servers-box" | "dns-button-box"
                | "dns-latency-box" | "dns-dot-box" => {},
                _ => continue,
            }
            for object_box_widget in object_box.children() {
                let widget_name = object_box_widget.widget_name();
                if let Ok(box_element_check) = object_box_widget.clone().downcast::<gtk::CheckButton>() {
                    let translated_text = crate::localization::get_locale_text(&widget_name);
                    box_element_check.set_label(&translated_text);
                } else if let Ok(box_element_btn) = object_box_widget.clone().downcast::<gtk::Button>() {
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

fn update_translation_options_section(section_box: &gtk::Box) {
    for section_box_element in section_box.children() {
        if let Ok(button_box) = section_box_element.clone().downcast::<gtk::Box>() {
            for button_box_widget in button_box.children() {
                let box_element_btn = button_box_widget.downcast::<gtk::Button>().unwrap();
                let widget_name = box_element_btn.widget_name().to_string();
                let translated_text = fl!("tweak-enabled-title", tweak = widget_name);
                box_element_btn.set_label(&translated_text);
            }
        } else if let Ok(section_label) = section_box_element.downcast::<gtk::Label>() {
            section_label.set_text(&fl!("tweaks"));
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

    let stack: gtk::Stack = builder.object("stack").unwrap();
    {
        if let Some(widget) = stack.child_by_name("tweaksBrowserpage") {
            if let Ok(viewport) = widget.downcast::<gtk::Viewport>() {
                let second_child =
                    &viewport.children()[0].clone().downcast::<gtk::Box>().unwrap().children()[1]
                        .clone()
                        .downcast::<gtk::Box>()
                        .unwrap();

                for second_child_child_widget in second_child.children() {
                    let second_child_child_box =
                        second_child_child_widget.downcast::<gtk::Box>().unwrap();

                    match second_child_child_box.widget_name().as_str() {
                        "tweaksBrowserpage_options" => {
                            update_translation_options_section(&second_child_child_box);
                        },
                        "tweaksBrowserpage_fixes" => {
                            update_translation_fixes_section(&second_child_child_box);
                        },
                        "tweaksBrowserpage_apps" => {
                            update_translation_apps_section(&second_child_child_box);
                        },
                        _ => panic!("Unknown widget!"),
                    }
                }
            }
        }
        if let Some(widget) = stack.child_by_name("dnsConnectionsBrowserpage") {
            if let Ok(viewport) = widget.downcast::<gtk::Viewport>() {
                let second_child =
                    &viewport.children()[0].clone().downcast::<gtk::Box>().unwrap().children()[1]
                        .clone()
                        .downcast::<gtk::Box>()
                        .unwrap();

                for second_child_child_widget in second_child.children() {
                    let second_child_child_box =
                        second_child_child_widget.downcast::<gtk::Box>().unwrap();
                    update_translation_connections_section(&second_child_child_box);
                }
            }
        }
    }
}
