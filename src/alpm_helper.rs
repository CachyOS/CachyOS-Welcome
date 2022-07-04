use crate::utils;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AlpmHelper {
    pub pkg_list_install: Vec<String>,
    pub pkg_list_removal: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AlpmHelperResult {
    NOTHING,
    REMOVE,
    ADD,
    BOTH,
}

impl AlpmHelper {
    pub fn new() -> Self {
        Self { pkg_list_install: Vec::new(), pkg_list_removal: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.pkg_list_install.clear();
        self.pkg_list_removal.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.pkg_list_install.is_empty() && self.pkg_list_removal.is_empty()
    }

    pub fn do_update(&self) -> AlpmHelperResult {
        let mut result = AlpmHelperResult::NOTHING;
        if self.pkg_list_install.is_empty() && self.pkg_list_removal.is_empty() {
            return result;
        }

        if !self.pkg_list_removal.is_empty() {
            if self.install_apps(&self.pkg_list_removal, false) {
                result = AlpmHelperResult::REMOVE;
            }
        }
        if !self.pkg_list_install.is_empty() {
            if self.install_apps(&self.pkg_list_install, true) {
                if result == AlpmHelperResult::NOTHING {
                    result = AlpmHelperResult::ADD;
                } else {
                    result = AlpmHelperResult::BOTH;
                }
            }
        }

        result
    }

    pub fn set_package(&mut self, pkg_name: &String, install: bool, installed: bool) {
        if self.to_remove(&pkg_name) {
            let index = self.pkg_list_removal.iter().position(|x| *x == *pkg_name).unwrap();
            self.pkg_list_removal.remove(index);
        } else if !install && installed {
            self.pkg_list_removal.push(String::from(pkg_name));
        }

        if self.to_install(&pkg_name) {
            let index = self.pkg_list_install.iter().position(|x| *x == *pkg_name).unwrap();
            self.pkg_list_install.remove(index);
        } else if install && !installed {
            self.pkg_list_install.push(String::from(pkg_name));
        }
    }

    pub fn to_install(&self, pkg_name: &String) -> bool {
        self.pkg_list_install.contains(&pkg_name)
    }

    pub fn to_remove(&self, pkg_name: &String) -> bool {
        self.pkg_list_removal.contains(&pkg_name)
    }

    fn install_apps(&self, pkg_list: &Vec<String>, install: bool) -> bool {
        let mut install_arg: &str = "-Sy";
        if pkg_list.is_empty() {
            return false;
        } else if !install {
            install_arg = "-R";
        }

        let packages_do = pkg_list.iter().map(|s| s.to_string() + " ").collect::<String>();
        let _ = utils::run_cmd_terminal(format!("pacman {} {}", install_arg, packages_do), true);
        match install {
            true => self.app_installed(&pkg_list[0]),
            false => !self.app_installed(&pkg_list[0]),
        }
    }

    fn app_installed(&self, pkg_name: &str) -> bool {
        let pacman =
            pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
        let alpm = alpm_utils::alpm_with_conf(&pacman).unwrap();
        match alpm.localdb().pkg(pkg_name) {
            Ok(_) => true,
            _ => false,
        }
    }
}
