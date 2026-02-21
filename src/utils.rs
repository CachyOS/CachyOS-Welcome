use crate::ui::RunCmdCallback;

use std::fs::File;
use std::path::Path;
use std::{fs, slice, str};

use gtk::prelude::*;

use subprocess::{Exec, ExitStatus};

#[derive(Debug)]
pub enum PacmanWrapper {
    Pak,
    Yay,
    Paru,
    Aura,
    Pacman,
}

#[inline]
pub fn fix_path(path: &str) -> String {
    if !path.starts_with('~') {
        return String::from(path);
    }
    path.replace('~', glib::home_dir().as_path().to_str().unwrap())
}

#[inline]
pub fn read_json(path: &str) -> serde_json::Value {
    let buf = fix_path(path);
    let data = fs::read_to_string(buf).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse")
}

#[inline]
pub fn write_json(path: &str, content: &serde_json::Value) {
    let output = File::create(fix_path(path)).expect("Unable to open file for writing");
    serde_json::to_writer(output, content).expect("Unable to write json to file");
}

#[inline]
pub const fn const_min(v1: usize, v2: usize) -> usize {
    if v1 <= v2 {
        v1
    } else {
        v2
    }
}

#[inline]
pub const fn string_substr(src_str: &str, pos: usize, n: usize) -> Result<&str, str::Utf8Error> {
    let rlen = const_min(n, src_str.len() - pos);
    let s = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(src_str.as_ptr().add(pos), rlen);

        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    };
    s
}

#[inline]
pub fn check_regular_file(path: &str) -> bool {
    let metadata = fs::metadata(path);
    if let Ok(meta) = metadata {
        meta.file_type().is_file()
    } else {
        false
    }
}

pub fn find_iter_in_model(
    model: &impl IsA<gtk::TreeModel>,
    search_text: &str,
) -> Option<gtk::TreeIter> {
    if let Some(iter) = model.iter_first() {
        loop {
            if let Ok(value) = model.value(&iter, 0).get::<String>() {
                if value == search_text {
                    return Some(iter);
                }
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    None
}

pub fn create_combo_with_model(group_store: &gtk::ListStore) -> gtk::ComboBoxText {
    let group_combo = gtk::ComboBoxText::builder().model(group_store).build();
    let combo_renderer = gtk::CellRendererText::new();
    group_combo.pack_start(&combo_renderer, true);
    group_combo.set_active(Some(0));

    group_combo
}

pub fn get_window_from_widget(passed_widget: &impl IsA<gtk::Widget>) -> Option<gtk::Window> {
    passed_widget.toplevel()?.downcast::<gtk::Window>().ok()
}

pub fn get_translation_msgid(objname: &str) -> &'static str {
    match objname {
        "autostartlabel" => "launch-start-label",
        "development" => "button-development-label",
        "software" => "button-software-label",
        "donate" => "button-donate-label",
        "forum" => "button-forum-label",
        "firstcategory" => "section-docs",
        "secondcategory" => "section-support",
        "thirdcategory" => "section-project",
        "install" => "button-installer-label",
        "installlabel" => "section-installer",
        "involved" => "button-involved-label",
        "readme" => "button-readme-label",
        "release" => "button-release-info-label",
        "welcomelabel" => "welcome-body",
        "welcometitle" => "welcome-title",
        "wiki" => "button-wiki-label",
        _ => panic!("unknown objname '{objname}'!"),
    }
}

pub fn run_cmd_terminal(callback: RunCmdCallback, cmd: String, escalate: bool) -> bool {
    callback(&cmd, escalate)
}

pub fn run_cmd(cmd: String, escalate: bool) -> anyhow::Result<ExitStatus> {
    if escalate {
        Ok(Exec::cmd("/sbin/pkexec").arg("bash").arg("-c").arg(cmd).join()?)
    } else {
        Ok(Exec::cmd("/sbin/bash").arg("-c").arg(cmd).join()?)
    }
}

#[inline]
pub fn get_pacman_wrapper() -> PacmanWrapper {
    if Path::new("/sbin/aura").exists() {
        return PacmanWrapper::Aura;
    } else if Path::new("/sbin/pak").exists() {
        return PacmanWrapper::Pak;
    } else if Path::new("/sbin/yay").exists() {
        return PacmanWrapper::Yay;
    } else if Path::new("/sbin/paru").exists() {
        return PacmanWrapper::Paru;
    }

    PacmanWrapper::Pacman
}

pub fn is_alpm_pkg_installed(package_name: &str) -> bool {
    let pacman = pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
    let alpm = alpm_utils::alpm_with_conf(&pacman).unwrap();
    alpm.localdb().pkg(package_name.as_bytes()).is_ok()
}

pub fn get_tweak_toggle_cmd(
    action_type: &str,
    action_data: &str,
    action_enabled: bool,
) -> (String, bool) {
    let sysaction = if action_enabled { "disable --now" } else { "enable --now" };
    let run_as_root = action_type != "user_service";

    let cmd = if !action_enabled {
        if run_as_root {
            format!("systemctl {sysaction} --force {action_data}")
        } else {
            format!("systemctl --user {sysaction} --force {action_data}")
        }
    } else if !run_as_root {
        format!("systemctl --user {sysaction} {action_data}")
    } else {
        format!("systemctl {sysaction} {action_data}")
    };

    (cmd, false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_file() {
        assert!(check_regular_file("/etc/fstab"));
        assert!(!check_regular_file("/etc"));
    }

    #[test]
    fn check_substr() {
        let text = "The test string is here";
        assert_eq!(string_substr(text, 0, 3).unwrap(), "The");
        assert_eq!(string_substr(text, 4, 4).unwrap(), "test");
        assert_eq!(string_substr(text, 19, 4).unwrap(), "here");
    }

    #[test]
    fn check_min() {
        assert_eq!(const_min(1, 2), 1);
        assert_eq!(const_min(2, 2), 2);
        assert_eq!(const_min(3, 2), 2);
    }
}
