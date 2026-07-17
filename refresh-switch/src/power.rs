use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use std::sync::mpsc::Sender;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerSource {
    Battery,
    Ac,
}

/// Query UPower for current power source.
pub fn get_power_source(conn: &Connection) -> Option<PowerSource> {
    let proxy = conn.with_proxy(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower/devices/DisplayDevice",
        Duration::from_secs(5),
    );

    // State: 1=charging, 2=discharging, 3=empty, 4=fully charged,
    //        5=pending charge, 6=pending discharge
    match proxy.get::<u32>("org.freedesktop.UPower.Device", "State") {
        Ok(state) => {
            let source = if state == 1 || state == 4 || state == 5 {
                PowerSource::Ac
            } else {
                PowerSource::Battery
            };
            Some(source)
        }
        Err(e) => {
            eprintln!("failed to get power state: {e}");
            None
        }
    }
}

/// Subscribe to UPower property changes on the DisplayDevice.
/// Sends a `()` on `tx` whenever a change is detected.
pub fn subscribe_power_changes(conn: &Connection, tx: Sender<()>) -> Result<(), dbus::Error> {
    let match_rule = dbus::message::MatchRule::new_signal(
        "org.freedesktop.DBus.Properties",
        "PropertiesChanged",
    )
    .with_path("/org/freedesktop/UPower/devices/DisplayDevice");

    conn.add_match(
        match_rule,
        move |_: (), _conn: &Connection, _msg: &dbus::message::Message| {
            let _ = tx.send(());
            true
        },
    )?;

    Ok(())
}
