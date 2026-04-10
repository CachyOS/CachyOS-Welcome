use std::path::{Path, PathBuf};

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TweakName {
    /// Profile Sync Daemon
    Psd,
    /// Systemd OOMD service
    Oomd,
    /// `BPFtune` service
    Bpftune,
    /// Bluetooth service
    Bluetooth,
    /// Ananicy Cpp service
    Ananicy,
    /// `CachyOS` update notifier
    #[clap(name = "cachy-update")]
    CachyUpdate,
    /// Install desktop booster services on supported GPUs
    #[clap(name = "gpu-boosters")]
    GpuBoosters,
}

pub fn get_details(tweak: TweakName) -> (&'static str, &'static str, &'static str) {
    match tweak {
        TweakName::Psd => ("user_service", "psd.service", "profile-sync-daemon"),
        TweakName::Oomd => ("service", "systemd-oomd.service", ""),
        TweakName::Bpftune => ("service", "bpftune.service", "bpftune-git"),
        TweakName::Bluetooth => ("service", "bluetooth.service", "bluez"),
        TweakName::Ananicy => ("service", "ananicy-cpp.service", "ananicy-cpp"),
        TweakName::CachyUpdate => {
            ("user_service", "arch-update.timer arch-update-tray.service", "cachy-update")
        },
        TweakName::GpuBoosters => ("package", "", "dmemcg-booster plasma-foreground-booster"),
    }
}

pub fn is_visible(tweak: TweakName) -> bool {
    match tweak {
        TweakName::GpuBoosters => crate::utils::has_intel_or_amd_gpu(),
        _ => true,
    }
}

pub fn are_packages_installed(package_names: &str) -> bool {
    package_names.split_whitespace().all(crate::utils::is_alpm_pkg_installed)
}

pub fn are_any_packages_installed(package_names: &str) -> bool {
    package_names.split_whitespace().any(crate::utils::is_alpm_pkg_installed)
}

/// Returns autostart desktop filenames associated with a tweak (legacy cleanup).
pub fn get_autostart_files(tweak: TweakName) -> &'static [&'static str] {
    match tweak {
        TweakName::CachyUpdate => &["arch-update-tray.desktop"],
        _ => &[],
    }
}

/// Returns the XDG autostart directory path.
pub fn get_autostart_dir() -> PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map_or_else(|_| glib::home_dir().join(".config"), PathBuf::from)
        .join("autostart")
}

/// Checks if any autostart desktop files exist for a tweak.
pub fn check_autostart_active(tweak: TweakName) -> bool {
    let autostart_dir = get_autostart_dir();
    get_autostart_files(tweak).iter().any(|f| autostart_dir.join(f).exists())
}

/// Removes autostart desktop files for a tweak.
pub fn remove_autostart_files(tweak: TweakName) {
    let autostart_dir = get_autostart_dir();
    for file in get_autostart_files(tweak) {
        let _ = std::fs::remove_file(autostart_dir.join(file));
    }
}

/// Checks if any of the given units are enabled globally.
pub fn is_globally_enabled(units: &str) -> bool {
    let global_dir = Path::new("/etc/systemd/user");
    let target_dirs = ["default.target.wants", "timers.target.wants", "sockets.target.wants"];
    units
        .split_whitespace()
        .any(|unit| target_dirs.iter().any(|dir| global_dir.join(dir).join(unit).exists()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpu_booster_tweak_has_expected_packages() {
        let (_, action_data, packages) = get_details(TweakName::GpuBoosters);
        assert_eq!(action_data, "");
        assert_eq!(packages, "dmemcg-booster plasma-foreground-booster");
    }

    #[test]
    fn non_gpu_tweaks_are_always_visible() {
        assert!(is_visible(TweakName::Psd));
        assert!(is_visible(TweakName::Oomd));
        assert!(is_visible(TweakName::Bluetooth));
    }
}
