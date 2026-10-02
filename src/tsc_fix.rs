//! Detects the AMD/Hygon "TSC warp" firmware bug (cores start with
//! out-of-sync TSCs, the kernel falls back to the slow HPET clocksource)
//! and offers the `tsc=directsync` workaround carried by `linux-cachyos`
//! (CachyOS/linux-cachyos#925). It is off by default upstream, so it is
//! only offered when this boot actually hit the warp.

use std::fs;
use std::path::Path;

use subprocess::{Exec, Redirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bootloader {
    Limine,
    SystemdBoot,
    Grub,
}

impl Bootloader {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Limine => "Limine",
            Self::SystemdBoot => "systemd-boot",
            Self::Grub => "GRUB",
        }
    }

    pub const fn config_path(self) -> &'static str {
        match self {
            Self::Limine => "/etc/default/limine",
            Self::SystemdBoot => "/etc/sdboot-manage.conf",
            Self::Grub => "/etc/default/grub",
        }
    }

    pub const fn regen_command(self) -> &'static str {
        match self {
            Self::Limine => "limine-mkinitcpio",
            Self::SystemdBoot => "sdboot-manage gen",
            Self::Grub => "grub-mkconfig -o /boot/grub/grub.cfg",
        }
    }

    /// Generated boot entries, checked after regenerating: a drop-in in
    /// `<config>.d/` or sdboot-manage's `OVERWRITE_EXISTING="no"` can
    /// silently discard our change to the main config.
    pub const fn generated_files(self) -> Option<&'static str> {
        match self {
            Self::Limine => None,
            Self::SystemdBoot => Some("\"$(bootctl -p)\"/loader/entries/*.conf"),
            Self::Grub => Some("/boot/grub/grub.cfg"),
        }
    }
}

pub fn detect_bootloader() -> Option<Bootloader> {
    if Path::new("/etc/default/limine").exists() {
        Some(Bootloader::Limine)
    } else if Path::new("/etc/sdboot-manage.conf").exists() {
        Some(Bootloader::SystemdBoot)
    } else if Path::new("/etc/default/grub").exists() {
        Some(Bootloader::Grub)
    } else {
        None
    }
}

fn cpuinfo_field<'a>(block: &'a str, key: &str) -> Option<&'a str> {
    block.lines().find_map(|line| {
        let (k, v) = line.split_once(':')?;
        (k.trim() == key).then(|| v.trim())
    })
}

/// AMD/Hygon without `tsc_adjust`, i.e. unable to fix a TSC offset itself.
fn is_affected_cpu(cpuinfo_block: &str) -> bool {
    let vendor_ok = cpuinfo_field(cpuinfo_block, "vendor_id")
        .is_some_and(|v| v == "AuthenticAMD" || v == "HygonGenuine");
    let Some(flags) = cpuinfo_field(cpuinfo_block, "flags") else { return false };
    let has_tsc_adjust = flags.split_whitespace().any(|f| f == "tsc_adjust");

    vendor_ok && !has_tsc_adjust
}

fn cpu_is_affected_family() -> bool {
    let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") else { return false };
    cpuinfo.split("\n\n").next().is_some_and(is_affected_cpu)
}

fn kernel_logged_tsc_warp() -> bool {
    Exec::cmd("journalctl")
        .args(["-k", "-b", "0", "-g", "TSC warp", "--no-pager", "-q"])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Null)
        .capture()
        .is_ok_and(|c| !c.stdout_str().trim().is_empty())
}

fn clocksource_is_hpet() -> bool {
    fs::read_to_string("/sys/devices/system/clocksource/clocksource0/current_clocksource")
        .is_ok_and(|c| c.trim() == "hpet")
}

fn kallsyms_has_symbol(kallsyms: &str, symbol: &str) -> bool {
    kallsyms.lines().any(|line| line.split_whitespace().nth(2) == Some(symbol))
}

/// The patch adds the global `tsc_allow_direct_sync`, so its presence in
/// kallsyms tells whether the running kernel has it, whatever its version.
/// Names stay visible under `kptr_restrict`, only addresses are zeroed.
fn kernel_binary_has_tsc_directsync() -> bool {
    fs::read_to_string("/proc/kallsyms")
        .is_ok_and(|k| kallsyms_has_symbol(&k, "tsc_allow_direct_sync"))
}

pub fn is_affected() -> bool {
    kernel_binary_has_tsc_directsync()
        && detect_bootloader().is_some()
        && cpu_is_affected_family()
        && kernel_logged_tsc_warp()
        && clocksource_is_hpet()
}

fn config_has_directsync(config: &str) -> bool {
    config.lines().filter(|line| !line.trim_start().starts_with('#')).any(|line| {
        line.split(|c: char| c.is_whitespace() || c == '"' || c == '\'')
            .any(|tok| tok == "tsc=directsync")
    })
}

/// The fix is in the bootloader config (rebooted since or not).
pub fn is_applied() -> bool {
    detect_bootloader()
        .and_then(|b| fs::read_to_string(b.config_path()).ok())
        .is_some_and(|c| config_has_directsync(&c))
}

/// The running kernel was booted with the fix, whatever the config says now.
pub fn is_active() -> bool {
    detect_bootloader().is_some()
        && fs::read_to_string("/proc/cmdline")
            .is_ok_and(|c| c.split_whitespace().any(|tok| tok == "tsc=directsync"))
}

/// Removes every `tsc=directsync` from the bootloader config and
/// regenerates it, failing loudly if any is left.
pub fn revert_commands(bootloader: Bootloader) -> String {
    format!(
        "set -e; \
         f={path}; \
         if grep -q 'tsc=directsync' \"$f\"; then \
             sed -i -E -e ':a' \
                 -e \"s/([\\\"'])tsc=directsync([\\\"'])/\\1\\2/\" \
                 -e \"s/([\\\"'=])tsc=directsync /\\1/\" \
                 -e \"s/ tsc=directsync([ \\\"']|\\$)/\\1/\" \
                 -e 'ta' \"$f\"; \
         fi; \
         if grep -v '^[[:space:]]*#' \"$f\" | grep -qw 'tsc=directsync'; then \
             echo \"tsc_fix: failed to remove tsc=directsync from $f\" >&2; exit 1; \
         fi; \
         {regen}{check}",
        path = bootloader.config_path(),
        regen = bootloader.regen_command(),
        check = bootloader.generated_files().map_or_else(String::new, |out| format!(
            "; if grep -qsw 'tsc=directsync' {out}; then \
                 echo \"tsc_fix: removed from $f, but the regenerated boot entries \
still have tsc=directsync; check {path}.d (and OVERWRITE_EXISTING for sdboot-manage)\" >&2; exit 1; \
             fi",
            path = bootloader.config_path(),
        )),
    )
}

/// Adds `tsc=directsync` to the kernel command line and regenerates the
/// bootloader config. Idempotent, and verifies the token actually landed,
/// since a `sed` that matches nothing still exits 0.
///
/// Limine: `KERNEL_CMDLINE[...]` is the *whole* command line, `root=`
/// included, so with no `[default]` line to extend this refuses rather
/// than write one holding only `tsc=directsync` (unbootable). Per-kernel
/// lines override `[default]` and get the token too; `fallback` ones are
/// left alone as a known-good way back in.
pub fn apply_commands(bootloader: Bootloader) -> String {
    match bootloader {
        Bootloader::Limine => String::from(
            "set -e; \
             f=/etc/default/limine; \
             if ! grep -Eq '^KERNEL_CMDLINE\\[default\\]\\+?=' \"$f\"; then \
                 echo \"tsc_fix: no KERNEL_CMDLINE[default] line in $f to extend; \
add tsc=directsync to it by hand\" >&2; exit 1; \
             fi; \
             sed -i -E \"/^KERNEL_CMDLINE[^=]*fallback/b; \
                 /^KERNEL_CMDLINE(\\[[^]]*\\])+\\+?=/ { \
                     /[\\\"'= ]tsc=directsync([\\\"' ]|\\$)/b; \
                     s/([\\\"'])[[:space:]]*\\$/ tsc=directsync\\1/; t; \
                     s/[[:space:]]*\\$/ tsc=directsync/; }\" \"$f\"; \
             grep -v '^[[:space:]]*#' \"$f\" | grep -qw 'tsc=directsync' || { \
                 echo \"tsc_fix: failed to add tsc=directsync to $f\" >&2; exit 1; \
             }; \
             limine-mkinitcpio",
        ),
        Bootloader::SystemdBoot => sourced_apply(bootloader, "LINUX_OPTIONS"),
        Bootloader::Grub => sourced_apply(bootloader, "GRUB_CMDLINE_LINUX_DEFAULT"),
    }
}

/// systemd-boot and GRUB: `key` holds only extra options (the generator
/// adds `root=` itself), so a fresh line is safe here, and it doesn't reach
/// the fallback/recovery entries.
fn sourced_apply(bootloader: Bootloader, key: &str) -> String {
    format!(
        "set -e; \
         f={path}; \
         if grep -Eq '^{key}\\+?=' \"$f\"; then \
             sed -i -E \"/^{key}\\+?=/ {{ \
                 /[\\\"'= ]tsc=directsync([\\\"' ]|\\$)/b; \
                 s/([\\\"'])[[:space:]]*\\$/ tsc=directsync\\1/; t; \
                 s/(\\+?=)([^\\\"'[:space:]]*)[[:space:]]*\\$/\\1\\\"\\2 tsc=directsync\\\"/; }}\" \"$f\"; \
         else \
             printf '{key}=\"tsc=directsync\"\\n' >> \"$f\"; \
         fi; \
         grep -v '^[[:space:]]*#' \"$f\" | grep -qw 'tsc=directsync' || {{ \
             echo \"tsc_fix: failed to add tsc=directsync to $f\" >&2; exit 1; \
         }}; \
         {regen}; \
         grep -qsw 'tsc=directsync' {out} || {{ \
             echo \"tsc_fix: added to $f, but the regenerated boot entries don't have \
tsc=directsync; check {path}.d (and OVERWRITE_EXISTING for sdboot-manage)\" >&2; exit 1; \
         }}",
        path = bootloader.config_path(),
        regen = bootloader.regen_command(),
        out = bootloader.generated_files().unwrap_or_default(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn affected_family_requires_amd_or_hygon_and_missing_tsc_adjust() {
        let amd_no_adjust = "vendor_id : AuthenticAMD\nflags : fpu vme de pse\n";
        let amd_with_adjust = "vendor_id : AuthenticAMD\nflags : fpu tsc_adjust de\n";
        let hygon_no_adjust = "vendor_id : HygonGenuine\nflags : fpu vme de\n";
        let intel = "vendor_id : GenuineIntel\nflags : fpu tsc_adjust de\n";
        let no_flags_line = "vendor_id : AuthenticAMD\n";

        assert!(is_affected_cpu(amd_no_adjust));
        assert!(!is_affected_cpu(amd_with_adjust));
        assert!(is_affected_cpu(hygon_no_adjust));
        assert!(!is_affected_cpu(intel));
        // Missing `flags` entirely: don't guess, stay conservative.
        assert!(!is_affected_cpu(no_flags_line));
    }

    #[test]
    fn cpuinfo_field_handles_varied_whitespace() {
        assert_eq!(cpuinfo_field("vendor_id\t: AuthenticAMD", "vendor_id"), Some("AuthenticAMD"));
        assert_eq!(cpuinfo_field("vendor_id : AuthenticAMD", "vendor_id"), Some("AuthenticAMD"));
        assert_eq!(cpuinfo_field("model name\t: Foo", "model name"), Some("Foo"));
        assert_eq!(cpuinfo_field("vendor_id : AMD", "model"), None);
    }

    #[test]
    fn kallsyms_matches_the_name_field_exactly() {
        let sample = "\
ffffffff81a2b3c0 B tsc_clocksource_reliable
ffffffff81a2b3c8 b tsc_allow_direct_sync
ffffffff81a2b3d0 b tsc_async_resets\n";

        assert!(kallsyms_has_symbol(sample, "tsc_allow_direct_sync"));
        assert!(!kallsyms_has_symbol(sample, "tsc_allow_direct_sync_x"));
        assert!(!kallsyms_has_symbol(sample, "tsc"));
        // zeroed address (kptr_restrict for an unprivileged reader): name
        // field is still there and still matches.
        let restricted = "0000000000000000 b tsc_allow_direct_sync\n";
        assert!(kallsyms_has_symbol(restricted, "tsc_allow_direct_sync"));
    }

    #[test]
    fn bootloader_names_are_stable() {
        assert_eq!(Bootloader::Limine.name(), "Limine");
        assert_eq!(Bootloader::SystemdBoot.name(), "systemd-boot");
        assert_eq!(Bootloader::Grub.name(), "GRUB");
    }

    #[test]
    fn apply_commands_reference_the_right_files() {
        assert!(apply_commands(Bootloader::Limine).contains("/etc/default/limine"));
        assert!(apply_commands(Bootloader::SystemdBoot).contains("/etc/sdboot-manage.conf"));
        assert!(apply_commands(Bootloader::Grub).contains("/etc/default/grub"));
    }

    #[test]
    fn config_has_directsync_matches_whole_token_outside_comments() {
        assert!(config_has_directsync(r#"KERNEL_CMDLINE[default]+="quiet tsc=directsync""#));
        assert!(config_has_directsync(r#"GRUB_CMDLINE_LINUX_DEFAULT="tsc=directsync quiet""#));
        assert!(!config_has_directsync(r#"KERNEL_CMDLINE[default]+="quiet rw""#));
        assert!(!config_has_directsync("# KERNEL_CMDLINE[default]=\"tsc=directsync\""));
        assert!(!config_has_directsync(r#"LINUX_OPTIONS="tsc=directsync_foo""#));
    }

    #[test]
    fn revert_commands_reference_the_right_files() {
        assert!(revert_commands(Bootloader::Limine).contains("/etc/default/limine"));
        assert!(revert_commands(Bootloader::SystemdBoot).contains("/etc/sdboot-manage.conf"));
        assert!(revert_commands(Bootloader::Grub).contains("/etc/default/grub"));
    }

    #[test]
    fn revert_commands_remove_token_in_every_position() {
        for (before, after) in [
            (
                r#"KERNEL_CMDLINE[default]+="quiet rw tsc=directsync""#,
                r#"KERNEL_CMDLINE[default]+="quiet rw""#,
            ),
            (
                r#"KERNEL_CMDLINE[default]="tsc=directsync quiet rw""#,
                r#"KERNEL_CMDLINE[default]="quiet rw""#,
            ),
            (
                r#"KERNEL_CMDLINE[default]="quiet tsc=directsync rw""#,
                r#"KERNEL_CMDLINE[default]="quiet rw""#,
            ),
            (r#"KERNEL_CMDLINE[default]="tsc=directsync""#, r#"KERNEL_CMDLINE[default]="""#),
            (
                r#"KERNEL_CMDLINE[default]="quiet tsc=directsync tsc=directsync""#,
                r#"KERNEL_CMDLINE[default]="quiet""#,
            ),
            ("KERNEL_CMDLINE[default]+='quiet tsc=directsync'", "KERNEL_CMDLINE[default]+='quiet'"),
            ("KERNEL_CMDLINE[default]+=quiet tsc=directsync", "KERNEL_CMDLINE[default]+=quiet"),
            ("KERNEL_CMDLINE[default]+=tsc=directsync quiet", "KERNEL_CMDLINE[default]+=quiet"),
            // Not there at all: a no-op, not an error.
            (r#"KERNEL_CMDLINE[default]="quiet rw""#, r#"KERNEL_CMDLINE[default]="quiet rw""#),
        ] {
            let mut file = tempfile::NamedTempFile::new().unwrap();
            std::io::Write::write_all(&mut file, format!("{before}\n").as_bytes()).unwrap();
            let path = file.path().to_str().unwrap();

            let script = revert_commands(Bootloader::Limine)
                .replace("/etc/default/limine", path)
                .replace("limine-mkinitcpio", "true"); // skip the real regen step

            let status = std::process::Command::new("bash")
                .arg("-c")
                .arg(&script)
                .status()
                .expect("failed to run test script");
            assert!(status.success(), "script failed against: {before}");

            let result = std::fs::read_to_string(path).unwrap();
            assert_eq!(result.trim_end(), after, "from: {before}");
        }
    }

    #[test]
    fn apply_commands_patches_both_assign_and_append_syntax() {
        // CachyOS's own /etc/default/limine uses `+=`, not `=`.
        for existing in [
            r#"KERNEL_CMDLINE[default]+="quiet rw root=UUID=abc""#,
            r#"KERNEL_CMDLINE[default]="quiet rw root=UUID=abc""#,
        ] {
            let mut file = tempfile::NamedTempFile::new().unwrap();
            std::io::Write::write_all(&mut file, existing.as_bytes()).unwrap();
            let path = file.path().to_str().unwrap();

            let script = apply_commands(Bootloader::Limine)
                .replace("/etc/default/limine", path)
                .replace("limine-mkinitcpio", "true"); // skip the real regen step

            let status = std::process::Command::new("bash")
                .arg("-c")
                .arg(&script)
                .status()
                .expect("failed to run test script");
            assert!(status.success(), "script failed against: {existing}");

            let result = std::fs::read_to_string(path).unwrap();
            assert!(result.contains("tsc=directsync"), "not patched: {result}");
        }
    }

    fn run_limine_apply(content: &str) -> (bool, String) {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();
        let path = file.path().to_str().unwrap();

        let script = apply_commands(Bootloader::Limine)
            .replace("/etc/default/limine", path)
            .replace("limine-mkinitcpio", "true");
        let status = std::process::Command::new("bash")
            .arg("-c")
            .arg(&script)
            .stderr(std::process::Stdio::null())
            .status()
            .expect("failed to run test script");
        (status.success(), std::fs::read_to_string(path).unwrap())
    }

    #[test]
    fn limine_apply_handles_every_quoting_style_exactly() {
        for (before, after) in [
            (
                "KERNEL_CMDLINE[default]+=\"quiet rw\"\n",
                "KERNEL_CMDLINE[default]+=\"quiet rw tsc=directsync\"\n",
            ),
            (
                "KERNEL_CMDLINE[default]+='quiet rw'\n",
                "KERNEL_CMDLINE[default]+='quiet rw tsc=directsync'\n",
            ),
            // limine-entry-tool doesn't need quotes at all.
            (
                "KERNEL_CMDLINE[default]+=quiet rw\n",
                "KERNEL_CMDLINE[default]+=quiet rw tsc=directsync\n",
            ),
            // Trailing whitespace after the closing quote.
            (
                "KERNEL_CMDLINE[default]+=\"quiet rw\"  \n",
                "KERNEL_CMDLINE[default]+=\"quiet rw tsc=directsync\"\n",
            ),
            // Commented-out mention doesn't count; fallback is left alone.
            (
                "#KERNEL_CMDLINE[default]+=\"tsc=directsync\"\n\
                 KERNEL_CMDLINE[default]+=\"quiet\"\n\
                 KERNEL_CMDLINE[fallback]+=\"quiet\"\n",
                "#KERNEL_CMDLINE[default]+=\"tsc=directsync\"\n\
                 KERNEL_CMDLINE[default]+=\"quiet tsc=directsync\"\n\
                 KERNEL_CMDLINE[fallback]+=\"quiet\"\n",
            ),
            // Trailing whitespace on an unquoted value.
            (
                "KERNEL_CMDLINE[default]+=quiet rw  \n",
                "KERNEL_CMDLINE[default]+=quiet rw tsc=directsync\n",
            ),
            // Per-kernel and per-profile lines too, fallback ones never.
            (
                "KERNEL_CMDLINE[default]+=\"quiet\"\n\
                 KERNEL_CMDLINE[linux-cachyos]=\"rw root=UUID=abc\"\n\
                 KERNEL_CMDLINE[linux-cachyos][gaming]+=\"mitigations=off\"\n\
                 KERNEL_CMDLINE[linux-cachyos-fallback]=\"rw root=UUID=abc\"\n\
                 KERNEL_CMDLINE[linux-cachyos][fallback]=\"rw\"\n",
                "KERNEL_CMDLINE[default]+=\"quiet tsc=directsync\"\n\
                 KERNEL_CMDLINE[linux-cachyos]=\"rw root=UUID=abc tsc=directsync\"\n\
                 KERNEL_CMDLINE[linux-cachyos][gaming]+=\"mitigations=off tsc=directsync\"\n\
                 KERNEL_CMDLINE[linux-cachyos-fallback]=\"rw root=UUID=abc\"\n\
                 KERNEL_CMDLINE[linux-cachyos][fallback]=\"rw\"\n",
            ),
            // Only the line that's missing it gets patched.
            (
                "KERNEL_CMDLINE[default]+=\"quiet tsc=directsync\"\n\
                 KERNEL_CMDLINE[linux-cachyos-lts]+=\"rw\"\n",
                "KERNEL_CMDLINE[default]+=\"quiet tsc=directsync\"\n\
                 KERNEL_CMDLINE[linux-cachyos-lts]+=\"rw tsc=directsync\"\n",
            ),
            // Already applied: untouched, no duplicate.
            (
                "KERNEL_CMDLINE[default]+=\"quiet tsc=directsync\"\n",
                "KERNEL_CMDLINE[default]+=\"quiet tsc=directsync\"\n",
            ),
        ] {
            let (ok, result) = run_limine_apply(before);
            assert!(ok, "script failed against: {before}");
            assert_eq!(result, after, "from: {before}");
        }
    }

    #[test]
    fn limine_apply_refuses_without_an_existing_cmdline() {
        // Writing a fresh [default] line here would drop root=.
        let before = "ESP_PATH=\"/boot\"\n#KERNEL_CMDLINE[default]+=rw root=UUID=...\n";
        let (ok, result) = run_limine_apply(before);
        assert!(!ok);
        assert_eq!(result, before);
    }

    /// Runs `script` against a temp copy of `config`, with `regen` standing
    /// in for the generator (`$f` = config, `$out` = generated file).
    fn run_sourced(
        bootloader: Bootloader,
        script: String,
        config: &str,
        regen: &str,
    ) -> (bool, String) {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, config.as_bytes()).unwrap();
        let path = file.path().to_str().unwrap();
        let out = tempfile::NamedTempFile::new().unwrap();
        let out_path = out.path().to_str().unwrap();

        let script = script
            .replace(bootloader.regen_command(), &format!("out={out_path}; {regen}"))
            .replace(bootloader.generated_files().unwrap(), out_path)
            .replace(bootloader.config_path(), path);
        let status = std::process::Command::new("bash")
            .arg("-c")
            .arg(&script)
            .stderr(std::process::Stdio::null())
            .status()
            .expect("failed to run test script");
        (status.success(), std::fs::read_to_string(path).unwrap())
    }

    /// The value of `key` once the config is sourced, as the generator sees it.
    fn sourced_value(config: &str, key: &str) -> String {
        let out = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("{config}\nprintf %s \"${key}\""))
            .output()
            .unwrap();
        String::from_utf8(out.stdout).unwrap()
    }

    // The generator faithfully passing the config through.
    const REGEN_OK: &str = "cp \"$f\" \"$out\"";
    // A drop-in or OVERWRITE_EXISTING=no discarding the change.
    const REGEN_OVERRIDDEN: &str = ": > \"$out\"";

    #[test]
    fn grub_and_sdboot_apply_produce_the_right_sourced_value() {
        for (bootloader, key) in [
            (Bootloader::Grub, "GRUB_CMDLINE_LINUX_DEFAULT"),
            (Bootloader::SystemdBoot, "LINUX_OPTIONS"),
        ] {
            for (before, value) in [
                (format!("{key}=\"loglevel=3 quiet\"\n"), "loglevel=3 quiet tsc=directsync"),
                (format!("{key}='loglevel=3 quiet'\n"), "loglevel=3 quiet tsc=directsync"),
                (format!("{key}=quiet\n"), "quiet tsc=directsync"),
                (format!("{key}=\"\"\n"), " tsc=directsync"),
                (format!("{key}=\"quiet\"  \n"), "quiet tsc=directsync"),
                // Commented mention doesn't count; no line at all is fine.
                (format!("#{key}=\"tsc=directsync\"\n"), "tsc=directsync"),
                (String::new(), "tsc=directsync"),
                // Already applied: unchanged, no duplicate.
                (format!("{key}=\"quiet tsc=directsync\"\n"), "quiet tsc=directsync"),
            ] {
                let (ok, after) =
                    run_sourced(bootloader, apply_commands(bootloader), &before, REGEN_OK);
                assert!(ok, "{key}: script failed against: {before:?}");
                assert_eq!(sourced_value(&after, key), value, "{key} from: {before:?}");
            }
        }
    }

    #[test]
    fn grub_apply_leaves_recovery_cmdline_alone() {
        let before = "GRUB_CMDLINE_LINUX_DEFAULT=\"quiet\"\nGRUB_CMDLINE_LINUX=\"\"\n";
        let (ok, after) =
            run_sourced(Bootloader::Grub, apply_commands(Bootloader::Grub), before, REGEN_OK);
        assert!(ok);
        assert_eq!(sourced_value(&after, "GRUB_CMDLINE_LINUX"), "");
    }

    #[test]
    fn sdboot_apply_leaves_fallback_options_alone() {
        let before = "LINUX_OPTIONS=\"quiet\"\nLINUX_FALLBACK_OPTIONS=\"\"\n";
        let (ok, after) = run_sourced(
            Bootloader::SystemdBoot,
            apply_commands(Bootloader::SystemdBoot),
            before,
            REGEN_OK,
        );
        assert!(ok);
        assert_eq!(sourced_value(&after, "LINUX_FALLBACK_OPTIONS"), "");
    }

    #[test]
    fn grub_and_sdboot_fail_loudly_when_generated_entries_miss_the_change() {
        for bootloader in [Bootloader::Grub, Bootloader::SystemdBoot] {
            let (ok, _) = run_sourced(bootloader, apply_commands(bootloader), "", REGEN_OVERRIDDEN);
            assert!(!ok, "{}: apply should fail", bootloader.name());

            // Revert where the generated entries keep the token anyway.
            let (ok, _) = run_sourced(
                bootloader,
                revert_commands(bootloader),
                "X=\"tsc=directsync\"\n",
                "echo tsc=directsync > \"$out\"",
            );
            assert!(!ok, "{}: revert should fail", bootloader.name());
        }
    }

    #[test]
    fn grub_and_sdboot_round_trip_on_stock_configs() {
        // Lines from the stock /etc/default/grub (grub 2:2.16) and
        // /etc/sdboot-manage.conf (systemd-boot-manager 21) on CachyOS.
        for (bootloader, stock) in [
            (
                Bootloader::Grub,
                "GRUB_DEFAULT=0\nGRUB_TIMEOUT=5\n\
                 GRUB_CMDLINE_LINUX_DEFAULT=\"loglevel=3 quiet\"\n\
                 GRUB_CMDLINE_LINUX=\"\"\nGRUB_DISABLE_RECOVERY=true\n",
            ),
            (
                Bootloader::SystemdBoot,
                "# Kernel options to be appended to the \"options\" line\n\
                 LINUX_OPTIONS=\"nowatchdog quiet splash\"\n\
                 #LINUX_FALLBACK_OPTIONS=\"\"\nREMOVE_EXISTING=\"no\"\n\
                 OVERWRITE_EXISTING=\"yes\"\n",
            ),
        ] {
            let (ok, applied) =
                run_sourced(bootloader, apply_commands(bootloader), stock, REGEN_OK);
            assert!(ok, "{}: apply failed", bootloader.name());
            assert_ne!(applied, stock);
            let (ok, reverted) =
                run_sourced(bootloader, revert_commands(bootloader), &applied, REGEN_OK);
            assert!(ok, "{}: revert failed", bootloader.name());
            assert_eq!(reverted, stock, "{}: round trip changed the file", bootloader.name());
        }
    }
}
