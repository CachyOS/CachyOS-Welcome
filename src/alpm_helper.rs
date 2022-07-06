use crate::utils;
use crate::utils::PacmanWrapper;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AlpmHelper {
    pub pkg_list_install: Vec<String>,
    pub pkg_list_removal: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlpmHelperResult {
    Nothing,
    Remove,
    Add,
    Both,
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
        let mut result = AlpmHelperResult::Nothing;
        if self.pkg_list_install.is_empty() && self.pkg_list_removal.is_empty() {
            return result;
        }

        if !self.pkg_list_removal.is_empty() && self.install_apps(&self.pkg_list_removal, false) {
            result = AlpmHelperResult::Remove;
        }
        if !self.pkg_list_install.is_empty() && self.install_apps(&self.pkg_list_install, true) {
            if result == AlpmHelperResult::Nothing {
                result = AlpmHelperResult::Add;
            } else {
                result = AlpmHelperResult::Both;
            }
        }

        result
    }

    pub fn set_package(&mut self, pkg_name: &String, install: bool, installed: bool) {
        if self.to_remove(pkg_name) {
            let index = self.pkg_list_removal.iter().position(|x| *x == *pkg_name).unwrap();
            self.pkg_list_removal.remove(index);
        } else if !install && installed {
            self.pkg_list_removal.push(String::from(pkg_name));
        }

        if self.to_install(pkg_name) {
            let index = self.pkg_list_install.iter().position(|x| *x == *pkg_name).unwrap();
            self.pkg_list_install.remove(index);
        } else if install && !installed {
            self.pkg_list_install.push(String::from(pkg_name));
        }
    }

    pub fn to_install(&self, pkg_name: &String) -> bool {
        self.pkg_list_install.contains(pkg_name)
    }

    pub fn to_remove(&self, pkg_name: &String) -> bool {
        self.pkg_list_removal.contains(pkg_name)
    }

    fn install_apps(&self, pkg_list: &Vec<String>, install: bool) -> bool {
        if pkg_list.is_empty() {
            return false;
        }
        let (cmd, escalate) = match install {
            true => match utils::get_pacman_wrapper_gui() {
                PacmanWrapper::Pamac => ("pamac-installer", false),
                PacmanWrapper::Pak => ("pak -Sy", false),
                PacmanWrapper::Yay => ("yay -Sy", false),
                PacmanWrapper::Paru => ("paru -Sy", false),
                _ => ("pacman -Sy", true),
            },
            false => match utils::get_pacman_wrapper_gui() {
                PacmanWrapper::Pamac => ("pamac-installer --remove", false),
                PacmanWrapper::Pak => ("pak -R", false),
                PacmanWrapper::Yay => ("yay -R", false),
                PacmanWrapper::Paru => ("paru -R", false),
                _ => ("pacman -R", true),
            },
        };

        let packages_do = pkg_list.iter().map(|s| s.to_string() + " ").collect::<String>();
        let _ = utils::run_cmd_terminal(format!("{} {}", cmd, packages_do), escalate);
        match install {
            true => self.app_installed(&pkg_list[0]),
            false => !self.app_installed(&pkg_list[0]),
        }
    }

    fn app_installed(&self, pkg_name: &str) -> bool {
        let pacman =
            pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
        let alpm = alpm_utils::alpm_with_conf(&pacman).unwrap();
        matches!(alpm.localdb().pkg(pkg_name), Ok(_))
    }
}
