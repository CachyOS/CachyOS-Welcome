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
        .arg(format!("--target={out_dir}/cachyos-welcome.gresource"))
        .arg("cachyos-welcome.gresource.xml")
        .status()
        .unwrap();

    if !status.success() {
        eprintln!("glib-compile-resources failed with exit status {status}");
        process::exit(1);
    }

    println!("cargo:rerun-if-changed=src/config.rs.in");
    println!("cargo:rerun-if-changed=src/config.rs");
    gen_config().expect("Failed to gen config.rs");

    if let Err(e) = gen_release_page() {
        eprintln!("cargo:warning=Failed to generate release page: {e}");
        write_fallback_release_page();
    }
}

fn gen_release_page() -> anyhow::Result<()> {
    let release_path = "data/pages/en/release";

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("cachyos-welcome-build")
        .build()?;

    let rss_bytes = match client.get("https://cachyos.org/rss.xml").send() {
        Ok(resp) => resp.bytes()?,
        Err(e) => {
            eprintln!("cargo:warning=RSS fetch failed: {e}");
            // Keep existing file if available, otherwise write fallback
            if !std::path::Path::new(release_path).exists() {
                write_fallback_release_page();
            }
            return Ok(());
        },
    };

    let mut reader = quick_xml::Reader::from_reader(rss_bytes.as_ref());
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut in_item = false;
    let mut current_tag = String::new();
    let mut title = String::new();
    let mut link = String::new();
    let mut description = String::new();
    let mut pub_date = String::new();
    let mut found = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "item" {
                    in_item = true;
                    title.clear();
                    link.clear();
                    description.clear();
                    pub_date.clear();
                } else if in_item {
                    current_tag = name;
                }
            },
            Ok(quick_xml::events::Event::Text(ref e)) if in_item => {
                let text = String::from_utf8_lossy(e.as_ref()).to_string();
                match current_tag.as_str() {
                    "title" => title = text,
                    "link" => link = text,
                    "description" => description = text,
                    "pubDate" => pub_date = text,
                    _ => {},
                }
            },
            Ok(quick_xml::events::Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "item" {
                    if title.to_lowercase().contains("release") {
                        found = true;
                        break;
                    }
                    in_item = false;
                }
                current_tag.clear();
            },
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {e}")),
            _ => {},
        }
        buf.clear();
    }

    if !found {
        eprintln!("cargo:warning=No release item found in RSS feed");
        write_fallback_release_page();
        return Ok(());
    }

    // Escape ampersands in description for valid Pango markup
    let description = description.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");

    let content = format!(
        "<big>{title}</big>\n\n{description}\n\n<i>{pub_date}</i>\n\n<a href=\"{link}\">For more \
         details</a>"
    );

    fs::create_dir_all("data/pages/en")?;
    let mut file = fs::File::create(release_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn write_fallback_release_page() {
    let content = "<big>Latest Release</big>\n\n\
                   Release information is currently unavailable.\n\n\
                   <a href=\"https://blog.cachyos.org\">Visit the CachyOS blog for the latest release notes</a>";

    fs::create_dir_all("data/pages/en").ok();
    if let Ok(mut file) = fs::File::create("data/pages/en/release") {
        let _ = file.write_all(content.as_bytes());
    }
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
    let output = Command::new("git").args(["rev-parse", "--short", "HEAD"]).output()?;
    Ok(String::from_utf8(output.stdout)?)
}
