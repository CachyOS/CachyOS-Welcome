use crate::power::PowerSource;

/// A display mode (resolution + refresh rate).
#[derive(Debug, Clone)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub rate: f64,
}

/// A connected monitor with its properties and available modes.
#[derive(Debug, Clone)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub rate: f64,
    pub scale: f64,
    pub pos_x: i32,
    pub pos_y: i32,
    pub modes: Vec<Mode>,
}

impl Monitor {
    /// Check if this is a built-in laptop panel (eDP connector).
    pub fn is_builtin(&self) -> bool {
        self.name.starts_with("eDP")
    }

    /// Get available refresh rates at the current resolution.
    pub fn rates_at_current_res(&self) -> Vec<f64> {
        self.modes
            .iter()
            .filter(|m| m.width == self.width && m.height == self.height)
            .map(|m| m.rate)
            .collect()
    }

    /// Pick the target refresh rate for the given power source.
    /// AC  -> highest available rate at current resolution.
    /// Battery -> lowest available rate at current resolution.
    pub fn target_rate(&self, source: PowerSource) -> Option<f64> {
        let rates = self.rates_at_current_res();
        match source {
            PowerSource::Ac => rates.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()),
            PowerSource::Battery => rates.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()),
        }
    }

    /// Check if switching is needed (current rate differs from target).
    pub fn needs_switch(&self, source: PowerSource) -> bool {
        if let Some(target) = self.target_rate(source) {
            (self.rate - target).abs() > 1.0
        } else {
            false
        }
    }
}
