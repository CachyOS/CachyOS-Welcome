# refresh-switch

Auto-switch display refresh rate on laptops based on power source (AC/battery).

## How it works

- Listens for UPower D-Bus signals (zero polling)
- **AC power**: switches built-in display to the highest available refresh rate
- **Battery**: switches to the lowest available refresh rate
- External monitors are never touched (they have their own power supply)
- Auto-detects desktop environment, monitor, and available refresh rates

## Supported desktops

| Desktop | Backend | Tool |
|---------|---------|------|
| Hyprland | `hyprctl monitors -j` / `hyprctl keyword monitor` | hyprctl |
| KDE Plasma | `kscreen-doctor -j` / `kscreen-doctor output...` | libkscreen |
| GNOME | `gnome-randr` or `gnome-monitor-config` | gnome-randr / gnome-monitor-config |

Desktop is auto-detected from `XDG_CURRENT_DESKTOP`.

## Installation

```bash
cargo build --release
cp target/release/refresh-switch ~/.local/bin/
cp refresh-switch.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now refresh-switch.service
```

## Usage

```bash
# Check status
systemctl --user status refresh-switch.service

# View logs
journalctl --user -u refresh-switch.service -f

# Run manually (for testing)
./target/release/refresh-switch
```

## Dependencies

- UPower (D-Bus) -- power state monitoring
- systemd -- service management
- One of the following (auto-detected):
  - **Hyprland**: `hyprctl`
  - **KDE Plasma**: `kscreen-doctor` (from `libkscreen`)
  - **GNOME**: `gnome-randr` or `gnome-monitor-config`

## Design

- Only built-in displays (eDP connectors) are switched
- Refresh rates are auto-detected from the monitor's supported modes
- Nothing is hardcoded -- works on any laptop with any resolution/refresh rates
- Lightweight: ~1MB memory, near-zero CPU usage
