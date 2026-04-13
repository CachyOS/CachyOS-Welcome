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
    }
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
