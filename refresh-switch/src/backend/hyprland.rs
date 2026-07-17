use super::{BackendError, DisplayBackend};
use crate::monitor::{Mode, Monitor};
use serde::Deserialize;
use std::process::Command;

pub struct HyprlandBackend;

/// JSON structure from `hyprctl monitors -j`.
#[derive(Deserialize)]
struct HyprMonitor {
    name: String,
    width: u32,
    height: u32,
    #[serde(rename = "refreshRate")]
    refresh_rate: f64,
    scale: f64,
    x: i32,
    y: i32,
    #[serde(rename = "availableModes")]
    available_modes: Vec<String>,
}

/// Parse a mode string like "2560x1600@240.00Hz" into a Mode.
fn parse_mode(s: &str) -> Option<Mode> {
    // format: "WIDTHxHEIGHT@RATE.RATEHz"
    let s = s.strip_suffix("Hz").unwrap_or(s);
    let (res, rate_str) = s.split_once('@')?;
    let (w, h) = res.split_once('x')?;
    Some(Mode {
        width: w.parse().ok()?,
        height: h.parse().ok()?,
        rate: rate_str.parse().ok()?,
    })
}

impl DisplayBackend for HyprlandBackend {
    fn name(&self) -> &str {
        "Hyprland"
    }

    fn get_monitors(&self) -> Result<Vec<Monitor>, BackendError> {
        let output = Command::new("hyprctl")
            .args(["monitors", "-j"])
            .output()
            .map_err(|e| BackendError::NotAvailable(format!("hyprctl: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackendError::CommandFailed(stderr.trim().to_string()));
        }

        let hypr_monitors: Vec<HyprMonitor> = serde_json::from_slice(&output.stdout)
            .map_err(|e| BackendError::ParseError(format!("hyprctl JSON: {e}")))?;

        let monitors = hypr_monitors
            .into_iter()
            .map(|hm| Monitor {
                name: hm.name,
                width: hm.width,
                height: hm.height,
                rate: hm.refresh_rate,
                scale: hm.scale,
                pos_x: hm.x,
                pos_y: hm.y,
                modes: hm
                    .available_modes
                    .iter()
                    .filter_map(|s| parse_mode(s))
                    .collect(),
            })
            .collect();

        Ok(monitors)
    }

    fn set_rate(&self, monitor: &Monitor, rate: f64) -> Result<(), BackendError> {
        // hyprctl keyword monitor NAME,WIDTHxHEIGHT@RATE,POSXxPOSY,SCALE
        let arg = format!(
            "{},{}x{}@{:.0},{}x{},{:.1}",
            monitor.name,
            monitor.width,
            monitor.height,
            rate,
            monitor.pos_x,
            monitor.pos_y,
            monitor.scale,
        );

        let output = Command::new("hyprctl")
            .args(["keyword", "monitor", &arg])
            .output()
            .map_err(|e| BackendError::CommandFailed(format!("hyprctl: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackendError::CommandFailed(stderr.trim().to_string()));
        }

        Ok(())
    }
}
