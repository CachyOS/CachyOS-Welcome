# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Welcome screen for CachyOS

# Tweaks page
tweaks = Tweaks
fixes = Utilities
applications = Applications
removed-db-lock = Pacman db lock was removed!
lock-doesnt-exist = Pacman db lock does not exist!
orphans-not-found = No orphan packages found!
package-not-installed = Package '{$package_name}' has not been installed!
gaming-package-installed = Gaming packages already installed!
winboat-package-installed = Winboat packages already installed!
vram-management-package-installed = VRAM management packages already installed!

# Application Browser page
advanced-btn = advanced
reset-btn = reset
update-system-app-btn = UPDATE SYSTEM
application-column = Application
description-column = Description
install-remove-column = Install/Remove
advanced-btn-tooltip = Toggle an extended selection of packages
reset-btn-tooltip = Reset your current selections...
update-system-app-btn-tooltip = Apply your current selections to the system

# Dns Connections page
dns-settings = DNS Settings
select-connection = Select Connection:
select-dns-server = Select DNS server:
apply = Apply
reset = Reset
enable-dot = Enable DNS over TLS (DoT)
dot-tooltip = Encrypt DNS queries using TLS for improved privacy (requires server support)
enable-doh = Enable DNS over HTTPS (DoH)
doh-tooltip = Encrypt DNS queries using HTTPS via blocky local proxy (requires server support, installs blocky)
doh-blocky-install-failed = Failed to install blocky for DoH support!
test-latency = Test Latency of Selected Server
test-latency-tooltip = Measure network latency to the selected DNS server
best-server = Select Best Server by Latency
best-server-tooltip = Test base DNS servers (excluding filtering variants) and select the fastest one
latency-result = {""}
server-info = {""}
latency-testing = testing...
latency-timeout = timeout
latency-no-result = no server responded
custom-dns = Custom
dhcp-automatic = DHCP (automatic)
custom-dns-ipv4 = IPv4 addresses (comma-separated):
custom-dns-ipv6 = IPv6 addresses (comma-separated):
custom-dns-dot-hostname = DoT hostname (optional):
custom-dns-invalid = Please enter at least an IPv4 or IPv6 address
custom-dns-invalid-hostname = Invalid DoT hostname
custom-dns-doh-url = DoH URL (for DNS over HTTPS):
custom-dns-doh-url-required = Please enter a valid DoH URL starting with https://
dns-check-hint = After applying, verify your DNS provider at
dns-server-changed = DNS server was successfully changed!
dns-server-failed = Failed to set DNS server!
dns-server-reset = DNS server has been reset!
dns-server-reset-failed = Failed to reset DNS server!
winboat-install-failed = Failed to install Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} enabled
tweak-psd-tooltip = Use RAM for browser profiles (faster, less disk wear)
tweak-oomd-tooltip = Proactively kill processes during low memory to prevent freezes
tweak-bpftune-tooltip = Automatically tune system network
tweak-bluetooth-tooltip = Enable support for Bluetooth wireless devices (mice, audio, etc.)
tweak-ananicycpp-tooltip = Auto-adjust process priorities for better system responsiveness
tweak-cachyupdate-tooltip = Update notifier in tray

# Tweaks page (fixes)
remove-lock-title = Remove db lock
reinstall-title = Reinstall all packages
reset-keyrings-title = Reset keyrings
update-system-title = System update
remove-orphans-title = Remove orphans
clear-pkgcache-title = Clear package cache
rankmirrors-title = Rank mirrors
dnsserver-title = Change DNS server
show-kwinw-debug-title = Show kwin(Wayland) debug window
install-gaming-title = Install Gaming packages
install-winboat-title = Install Winboat
install-vram-management-title = Install VRAM Management
install-vram-management-tooltip = Prioritize VRAM for the foreground application so the GPU driver avoids spilling buffers into system RAM (GTT).

# Main Page (buttons)
button-about-tooltip = About
button-web-resource-tooltip = Web resource
button-development-label = Development
button-software-label = Software
button-donate-label = Donate
button-forum-label = Forum
button-installer-label = Launch installer
button-involved-label = Get involved
button-readme-label = Read me
button-release-info-label = Release info
button-wiki-label = Wiki

# Main Page (sections)
section-docs = DOCUMENTATION
section-installer = INSTALLATION
section-support = SUPPORT
section-project = PROJECT

# Main Page (launch installer)
recommended = recommended
calamares-install-type = Calamares install type

# Main Page (body)
offline-error = Unable to start online installation! No internet connection
unsupported-hw-warning = You are attempting to install on hardware not supported by the current ISO, your installation will not be eligible for support
desktop-on-handheld-error = You are attempting to install the Desktop edition on a handheld device. Please use the Handheld edition for proper support on this hardware
outdated-version-warning = You are using an older version of CachyOS ISO, please consider using latest version for installations
testing-iso-warning = You are using a testing ISO, testing ISOs are not considered stable and ready for use
tweaksbrowser-label = Apps/Tweaks
appbrowser-label = Install Apps
launch-start-label = Launch at start
welcome-title = Welcome to CachyOS!
welcome-body =
    Thank you for joining our community!

    We, the CachyOS Developers, hope that you will enjoy using CachyOS as much as we enjoy building it. The links below will help you get started with your new operating system. So enjoy the experience, and don't hesitate to send us your feedback.
