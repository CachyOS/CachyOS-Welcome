use crate::{check_regular_file, fl, utils};

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::Deserialize;
use subprocess::{Exec, Redirection};
use tracing::{error, info};

#[derive(Deserialize)]
struct Versions {
    #[serde(rename = "desktopISOVersion")]
    desktop_iso_version: String,
    #[serde(rename = "handheldISOVersion")]
    handheld_iso_version: String,
}

pub fn is_iso(preferences: &serde_json::Value) -> bool {
    Path::new(&preferences["live_path"].as_str().unwrap()).exists()
        && check_regular_file(preferences["installer_path"].as_str().unwrap())
}
