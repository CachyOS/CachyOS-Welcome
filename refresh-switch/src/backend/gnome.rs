use super::{BackendError, DisplayBackend};
use crate::monitor::{Mode, Monitor};

pub struct GnomeBackend;

impl GnomeBackend {
    /// Query monitors via gnome-randr or gnome-monitor-config CLI tools.
    fn query_monitors() -> Result<Vec<GnomeMonitor>, BackendError> {
        // Try gnome-randr first (simpler output)
        if let Ok(out) = std::process::Command::new("gnome-randr")
            .arg("list")
            .output()
        {
            if out.status.success() {
                return parse_monitor_list(&String::from_utf8_lossy(&out.stdout));
            }
        }

        // Fall back to gnome-monitor-config
        let output = std::process::Command::new("gnome-monitor-config")
            .arg("list")
            .output()
            .map_err(|e| {
                BackendError::NotAvailable(format!(
                    "GNOME: need gnome-randr or gnome-monitor-config: {e}"
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackendError::CommandFailed(stderr.trim().to_string()));
        }

        parse_monitor_list(&String::from_utf8_lossy(&output.stdout))
    }
}

/// Intermediate structs for GNOME monitor data.
#[derive(Debug, Clone)]
struct GnomeMonitor {
    connector: String,
    modes: Vec<GnomeMode>,
}

#[derive(Debug, Clone)]
struct GnomeMode {
    width: u32,
    height: u32,
    rate: f64,
    is_current: bool,
}

/// Parse output of `gnome-monitor-config list` or `gnome-randr list`.
///
/// Example output:
/// ```text
/// Monitor [ eDP-1 ] ON
///   2560x1600@240.000 [current] [preferred]
///   2560x1600@60.000
/// Monitor [ HDMI-1 ] ON
///   1920x1080@60.000 [current] [preferred]
/// ```
fn parse_monitor_list(output: &str) -> Result<Vec<GnomeMonitor>, BackendError> {
    let mut monitors = Vec::new();
    let mut current_monitor: Option<GnomeMonitor> = None;

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("Monitor") {
            if let Some(mon) = current_monitor.take() {
                monitors.push(mon);
            }

            // Parse "Monitor [ eDP-1 ] ON"
            if let Some(name) = trimmed
                .split('[')
                .nth(1)
                .and_then(|s| s.split(']').next())
                .map(|s| s.trim().to_string())
            {
                current_monitor = Some(GnomeMonitor {
                    connector: name,
                    modes: Vec::new(),
                });
            }
        } else if let Some(ref mut mon) = current_monitor {
            if let Some(mode) = parse_mode_line(trimmed) {
                mon.modes.push(mode);
            }
        }
    }

    if let Some(mon) = current_monitor {
        monitors.push(mon);
    }

    Ok(monitors)
}

/// Parse a mode line like "2560x1600@240.000 [current] [preferred]".
fn parse_mode_line(line: &str) -> Option<GnomeMode> {
    let line = line.trim();
    if line.is_empty() || line.starts_with("Monitor") {
        return None;
    }

    let is_current = line.contains("[current]");

    // Extract the mode spec (everything before '[')
    let spec = line.split('[').next()?.trim();

    // Parse "WIDTHxHEIGHT@RATE"
    let (res, rate_str) = spec.split_once('@')?;
    let (w, h) = res.split_once('x')?;

    Some(GnomeMode {
        width: w.parse().ok()?,
        height: h.parse().ok()?,
        rate: rate_str.parse().ok()?,
        is_current,
    })
}

impl DisplayBackend for GnomeBackend {
    fn name(&self) -> &str {
        "GNOME"
    }

    fn get_monitors(&self) -> Result<Vec<Monitor>, BackendError> {
        let gnome_monitors = Self::query_monitors()?;

        let monitors = gnome_monitors
            .into_iter()
            .filter_map(|gm| {
                let current = gm.modes.iter().find(|m| m.is_current)?;

                Some(Monitor {
                    name: gm.connector,
                    width: current.width,
                    height: current.height,
                    rate: current.rate,
                    scale: 1.0, // gnome-randr/gnome-monitor-config don't expose this easily
                    pos_x: 0,
                    pos_y: 0,
                    modes: gm
                        .modes
                        .iter()
                        .map(|m| Mode {
                            width: m.width,
                            height: m.height,
                            rate: m.rate,
                        })
                        .collect(),
                })
            })
            .collect();

        Ok(monitors)
    }

    fn set_rate(&self, monitor: &Monitor, rate: f64) -> Result<(), BackendError> {
        // Find the exact rate from the monitor's mode list to avoid floating-point
        // formatting mismatches. GNOME tools do exact string matching on mode specs,
        // so "165.002" won't match a mode reported as "165.00195312".
        let exact_rate = monitor
            .modes
            .iter()
            .filter(|m| m.width == monitor.width && m.height == monitor.height)
            .min_by(|a, b| {
                let da = (a.rate - rate).abs();
                let db = (b.rate - rate).abs();
                da.partial_cmp(&db).unwrap()
            })
            .map(|m| m.rate)
            .unwrap_or(rate);

        // Format as integer Hz — both gnome-randr and gnome-monitor-config accept
        // integer rates and match to the closest available mode, same as Hyprland/KDE.
        let mode_str = format!("{}x{}@{:.0}", monitor.width, monitor.height, exact_rate);

        // Try gnome-randr first
        if let Ok(out) = std::process::Command::new("gnome-randr")
            .args(["modify", &monitor.name, "-m", &mode_str])
            .output()
        {
            if out.status.success() {
                return Ok(());
            }
        }

        // Fall back to gnome-monitor-config
        // Use -LM flags only (-p is not supported on all versions)
        let output = std::process::Command::new("gnome-monitor-config")
            .args(["set", "-LM", &monitor.name, "-m", &mode_str])
            .output()
            .map_err(|e| {
                BackendError::NotAvailable(format!(
                    "GNOME: need gnome-randr or gnome-monitor-config: {e}"
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackendError::CommandFailed(stderr.trim().to_string()));
        }

        Ok(())
    }
}
