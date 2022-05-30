#[derive(Clone, Debug)]
#[repr(C)]
pub struct HelloWindow {
    pub builder: gtk::Builder,
    pub window: gtk::Window,
    pub preferences: serde_json::Value,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SystemdUnits {
    pub loaded_units: Vec<String>,
    pub enabled_units: Vec<String>,
}

impl SystemdUnits {
    pub fn new() -> Self {
        Self { loaded_units: Vec::new(), enabled_units: Vec::new() }
    }
}

impl Default for SystemdUnits {
    fn default() -> Self {
        Self::new()
    }
}
