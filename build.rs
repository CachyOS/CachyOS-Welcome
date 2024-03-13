use std::io::Write;
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

    println!("cargo:rerun-if-changed=src/config.rs.in");
    println!("cargo:rerun-if-changed=src/config.rs");
    gen_config().expect("Failed to gen config.rs");
}

fn gen_config() -> anyhow::Result<()> {
    let base_id = "org.cachyos.hello";

    let version = env::var("CARGO_PKG_VERSION")?;

    let (profile, version_suffix, app_id) = if env::var("PROFILE")? == "debug" {
        let profile = "Devel";
        let vcs_tag = get_vcs_tag()?;
        let version_suffix =
            if vcs_tag.is_empty() { "-devel".to_string() } else { format!("-{vcs_tag}") };
        (profile.to_owned(), version_suffix, format!("{base_id}.{profile}"))
    } else {
        (String::new(), String::new(), base_id.to_owned())
    };

    let final_config = fs::read_to_string("src/config.rs.in")?
        .replace("@APP_ID@", &format!("\"{app_id}\""))
        .replace("@PROFILE@", &format!("\"{profile}\""))
        .replace("@VERSION@", &format!("\"{version}{version_suffix}\""));

    let mut file = fs::File::create("src/config.rs")?;
    file.write_all(final_config.as_bytes())?;
    Ok(())
}

fn get_vcs_tag() -> anyhow::Result<String> {
    let output = Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output()?;
    Ok(String::from_utf8(output.stdout)?)
}
