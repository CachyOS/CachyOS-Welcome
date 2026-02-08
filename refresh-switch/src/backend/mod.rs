pub mod gnome;
pub mod hyprland;
pub mod kde;

use crate::monitor::Monitor;
use std::fmt;

/// Errors from display backends.
#[derive(Debug)]
pub enum BackendError {
    /// The required tool is not available (e.g. hyprctl, kscreen-doctor).
    NotAvailable(String),
    /// Command execution failed.
    CommandFailed(String),
    /// Failed to parse output.
    ParseError(String),
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAvailable(msg) => write!(f, "backend not available: {msg}"),
            Self::CommandFailed(msg) => write!(f, "command failed: {msg}"),
            Self::ParseError(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

/// Trait implemented by each compositor/DE backend.
pub trait DisplayBackend {
    /// Human-readable name of this backend.
    fn name(&self) -> &str;

    /// Query all connected monitors.
    fn get_monitors(&self) -> Result<Vec<Monitor>, BackendError>;

    /// Set refresh rate on a specific monitor, preserving resolution/scale/position.
    fn set_rate(&self, monitor: &Monitor, rate: f64) -> Result<(), BackendError>;
}

/// Supported desktop environments / compositors.
#[derive(Debug, Clone, Copy)]
pub enum Desktop {
    Hyprland,
    Kde,
    Gnome,
}

/// Auto-detect the running desktop environment from XDG_CURRENT_DESKTOP.
pub fn detect_desktop() -> Option<Desktop> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let desktop = desktop.to_lowercase();

    // XDG_CURRENT_DESKTOP can contain multiple values separated by ':'
    for entry in desktop.split(':') {
        match entry.trim() {
            "hyprland" => return Some(Desktop::Hyprland),
            "kde" => return Some(Desktop::Kde),
            "gnome" => return Some(Desktop::Gnome),
            _ => {}
        }
    }

    None
}

/// Create the appropriate backend for the detected (or specified) desktop.
pub fn create_backend(desktop: Desktop) -> Box<dyn DisplayBackend> {
    match desktop {
        Desktop::Hyprland => Box::new(hyprland::HyprlandBackend),
        Desktop::Kde => Box::new(kde::KdeBackend),
        Desktop::Gnome => Box::new(gnome::GnomeBackend),
    }
}
