use super::{BackendError, DisplayBackend};
use crate::monitor::{Mode, Monitor};
use serde::Deserialize;
use std::process::Command;

pub struct KdeBackend;

/// JSON structure from `kscreen-doctor -j`.
#[derive(Deserialize)]
struct KscreenOutput {
    outputs: Vec<KscreenMonitor>,
}

#[derive(Deserialize)]
struct KscreenMonitor {
    name: String,
    enabled: bool,
    #[serde(rename = "currentModeId")]
    current_mode_id: Option<String>,
    pos: Option<KscreenPos>,
    scale: Option<f64>,
    modes: Vec<KscreenMode>,
}

#[derive(Deserialize)]
struct KscreenPos {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct KscreenMode {
    id: String,
    #[serde(rename = "refreshRate")]
    refresh_rate: f64,
    size: KscreenSize,
}

#[derive(Deserialize)]
struct KscreenSize {
    width: u32,
    height: u32,
}

impl DisplayBackend for KdeBackend {
    fn name(&self) -> &str {
        "KDE Plasma"
    }

    fn get_monitors(&self) -> Result<Vec<Monitor>, BackendError> {
        let output = Command::new("kscreen-doctor")
            .arg("-j")
            .output()
            .map_err(|e| BackendError::NotAvailable(format!("kscreen-doctor: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackendError::CommandFailed(stderr.trim().to_string()));
        }

        let kscreen: KscreenOutput = serde_json::from_slice(&output.stdout)
            .map_err(|e| BackendError::ParseError(format!("kscreen-doctor JSON: {e}")))?;

        let monitors = kscreen
            .outputs
            .into_iter()
            .filter(|o| o.enabled)
            .filter_map(|o| {
                let current_mode = o
                    .current_mode_id
                    .as_ref()
                    .and_then(|id| o.modes.iter().find(|m| &m.id == id))?;

                let pos = o.pos.as_ref();

                Some(Monitor {
                    name: o.name,
                    width: current_mode.size.width,
                    height: current_mode.size.height,
                    rate: current_mode.refresh_rate,
                    scale: o.scale.unwrap_or(1.0),
                    pos_x: pos.map_or(0, |p| p.x),
                    pos_y: pos.map_or(0, |p| p.y),
                    modes: o
                        .modes
                        .iter()
                        .map(|m| Mode {
                            width: m.size.width,
                            height: m.size.height,
                            rate: m.refresh_rate,
                        })
                        .collect(),
                })
            })
            .collect();

        Ok(monitors)
    }

    fn set_rate(&self, monitor: &Monitor, rate: f64) -> Result<(), BackendError> {
        // kscreen-doctor output.NAME.mode.WIDTHxHEIGHT@RATE
        let arg = format!(
            "output.{}.mode.{}x{}@{:.0}",
            monitor.name, monitor.width, monitor.height, rate,
        );

        let output = Command::new("kscreen-doctor")
            .arg(&arg)
            .output()
            .map_err(|e| BackendError::CommandFailed(format!("kscreen-doctor: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackendError::CommandFailed(stderr.trim().to_string()));
        }

        Ok(())
    }
}
