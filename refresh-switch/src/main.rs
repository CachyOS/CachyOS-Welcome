mod backend;
mod monitor;
mod power;

use backend::{create_backend, detect_desktop, DisplayBackend};
use dbus::blocking::Connection;
use power::PowerSource;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;

static RUNNING: AtomicBool = AtomicBool::new(true);

/// Apply the correct refresh rate to all built-in monitors.
fn apply(conn: &Connection, backend: &dyn DisplayBackend, last_state: &mut Option<PowerSource>) {
    let Some(source) = power::get_power_source(conn) else {
        return;
    };

    // Skip if power source hasn't changed
    if *last_state == Some(source) {
        return;
    }

    let source_name = match source {
        PowerSource::Ac => "AC",
        PowerSource::Battery => "battery",
    };

    eprintln!("power: switched to {source_name}");

    let monitors = match backend.get_monitors() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("failed to query monitors: {e}");
            return;
        }
    };

    // Only switch built-in (eDP) displays
    let builtin: Vec<_> = monitors.iter().filter(|m| m.is_builtin()).collect();

    if builtin.is_empty() {
        eprintln!("no built-in displays found, skipping");
        *last_state = Some(source);
        return;
    }

    for mon in &builtin {
        let Some(target) = mon.target_rate(source) else {
            eprintln!("  {}: no suitable rate found", mon.name);
            continue;
        };

        if !mon.needs_switch(source) {
            eprintln!("  {}: already at {:.0} Hz", mon.name, mon.rate);
            continue;
        }

        eprintln!("  {}: {:.0} Hz -> {:.0} Hz", mon.name, mon.rate, target);
        if let Err(e) = backend.set_rate(mon, target) {
            eprintln!("  {}: failed to set rate: {e}", mon.name);
        }
    }

    *last_state = Some(source);
}

fn main() {
    // Detect desktop environment
    let desktop = match detect_desktop() {
        Some(d) => d,
        None => {
            let xdg = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
            eprintln!("unsupported desktop: {xdg}");
            eprintln!("supported: Hyprland, KDE Plasma, GNOME");
            std::process::exit(1);
        }
    };

    let backend = create_backend(desktop);
    eprintln!("refresh-switch: backend={}", backend.name());

    // Connect to system bus for UPower
    let conn = Connection::new_system().expect("failed to connect to D-Bus");

    // Apply initial state
    let mut last_state: Option<PowerSource> = None;
    apply(&conn, backend.as_ref(), &mut last_state);

    // Setup signal handling
    let (tx, rx): (Sender<()>, _) = channel();
    let tx_ctrlc = tx.clone();

    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::SeqCst);
        let _ = tx_ctrlc.send(());
    })
    .expect("failed to set signal handler");

    // Subscribe to UPower power state changes
    let tx_signal = tx.clone();
    power::subscribe_power_changes(&conn, tx_signal).expect("failed to subscribe to UPower");

    eprintln!("listening for power changes...");

    while RUNNING.load(Ordering::SeqCst) {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(()) => {
                apply(&conn, backend.as_ref(), &mut last_state);
            }
            Err(_) => {
                conn.process(Duration::from_millis(10)).ok();
            }
        }
    }

    eprintln!("shutting down");
}
