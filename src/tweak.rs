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
    /// Auto-switch display refresh rate based on power source (AC/battery)
    #[clap(name = "refresh-switch")]
    RefreshSwitch,
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
        TweakName::RefreshSwitch => ("user_service", "refresh-switch.service", "refresh-switch"),
    }
}
