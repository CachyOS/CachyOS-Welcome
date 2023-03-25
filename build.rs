use std::process::{self, Command};
use std::{env, fs};

fn main() {
    for i in fs::read_dir("data").unwrap() {
        println!("cargo:rerun-if-changed={}", i.unwrap().path().display());
    }
    for i in fs::read_dir("ui").unwrap() {
        println!("cargo:rerun-if-changed={}", i.unwrap().path().display());
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new("glib-compile-resources")
        .arg(&format!("--target={}/cachyos-welcome.gresource", out_dir))
        .arg("cachyos-welcome.gresource.xml")
        .status()
        .unwrap();

    if !status.success() {
        eprintln!("glib-compile-resources failed with exit status {}", status);
        process::exit(1);
    }
}
