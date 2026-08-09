# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Willkommensdialog für CachyOS

# Tweaks page
tweaks = Optimierungen
fixes = Dienstprogramme
applications = Anwendungen
removed-db-lock = Pacman db lock wurde entfernt!
lock-doesnt-exist = Pacman db lock existiert nicht!
orphans-not-found = Keine verwaisten Pakete gefunden!
package-not-installed = Das Paket '{$package_name}' wurde nicht installiert!
gaming-package-installed = Gaming-Pakete sind schon installiert!
winboat-package-installed = Winboat-Pakete sind schon installiert!
vram-management-package-installed = VRAM-Verwaltungspakete sind schon installiert!

# Troubleshooting page
troubleshooting = Problembehandlung

# Dns Connections page
dns-settings = DNS-Einstellungen
select-connection = Verbindung auswählen:
select-dns-server = DNS-Server auswählen:
apply = Anwenden
reset = Zurücksetzen
enable-encrypted-dns = DNS über {$protocol} ({$abbr}) aktivieren
blocky-install-failed = Die Installation von blocky für {$mode}-Unterstützung ist fehlgeschlagen!
test-latency = Latenz des ausgewählten Servers testen
test-latency-tooltip = Netzwerklatenz zum ausgewählten DNS-Server messen
best-server = Besten Server nach Latenz wählen
best-server-tooltip = Basis-DNS-Server testen (ohne Filtervarianten) und den schnellsten auswählen
latency-result = {""}
server-info = {""}
latency-testing = teste...
latency-timeout = Zeitüberschreitung
latency-no-result = kein Server hat geantwortet
custom-dns = Benutzerdefiniert
dhcp-automatic = DHCP (automatisch)
custom-dns-ip = {$version}-Adressen (Komma-getrennt):
custom-dns-dot-hostname = DoT-Hostname (optional):
custom-dns-invalid = Bitte gib mindestens eine IPv4- oder IPv6- Adresse ein
custom-dns-invalid-hostname = Ungültiger DoT-Hostname
custom-dns-doh-url = DoH URL (für DNS über HTTPS):
custom-dns-doh-url-required = Gib bitte eine gültige DoH-URL ein, die mit https:// beginnt
custom-dns-doq-endpoint = DoQ-Endpunkt (für DNS über QUIC):
custom-dns-doq-endpoint-required = Gib bitte einen gültigen DoQ-Endpunkt ein, der mit quic: oder quic:// beginnt 
dns-check-hint = Nach dem Anwenden, überprüfe deinen DNS-Anbieter auf {$dnscheck_url}
dns-server-changed = DNS Server wurde erfolgreich geändert!
dns-server-failed = DNS-Server konnte nicht eingestellt werden!
dns-server-reset = DNS-Server wurde zurückgesetzt!
dns-server-reset-failed = DNS-Server konnte nicht zurückgesetzt werden!
winboat-install-failed = Winboat konnte nicht installiert werden!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} aktiviert
tweak-psd-tooltip = RAM für Browserprofile verwenden (schneller, weniger Festplattenverschleiß)
tweak-oomd-tooltip = Proaktiv Prozesse bei geringem Arbeitsspeicher beenden, um Abstürze zu verhindern
tweak-bpftune-tooltip = Systemnetzwerk automatisch optimieren
tweak-bluetooth-tooltip = Unterstützung für drahtlose Bluetooth-Geräte (Mäuse, Audio, etc.)
tweak-ananicycpp-tooltip = Automatische Anpassung der Prozessprioritäten für eine bessere Systemreaktionsfähigkeit
tweak-cachyupdate-tooltip = Update-Benachrichtigungsdienst im Benachrichtigungsfeld

# Tweaks page (fixes)
remove-lock-title = Datenbanksperre entfernen
reinstall-title = Alle Pakete neu installieren
reset-keyrings-title = Keyrings zurücksetzen
update-system-title = System-Aktualisierung
remove-orphans-title = Nicht verwendete Pakete entfernen
clear-pkgcache-title = Paket-Cache löschen
rankmirrors-title = Spiegelserver (Mirrors) bewerten
dnsserver-title = DNS-Server ändern
show-kwinw-debug-title = kwin(Wayland)-Debug-Fenster anzeigen
install-gaming-title = Gaming-Pakete installieren
install-winboat-title = Winboat installieren
install-vram-management-title = VRAM-Verwaltung installieren
install-vram-management-tooltip = Priorisiere VRAM für die Vordergrundanwendung, damit der GPU-Treiber das Auslagern von Puffern in den System-RAM (GTT) vermeidet.

# Main Page (buttons)
button-about-tooltip = Über
button-web-resource-tooltip = Web-Ressource
button-development-label = Entwicklung
button-software-label = Software
button-donate-label = Spenden
button-forum-label = Forum
button-installer-label = Installation starten
button-involved-label = Mitmachen
button-readme-label = Lies mich
button-release-info-label = Versionshinweise
button-wiki-label = Wiki

# Main Page (sections)
section-docs = DOKUMENTATION
section-installer = INSTALLATION
section-support = UNTERSTÜTZUNG
section-project = PROJEKT

# Main Page (body)
offline-error = Die Online-Installation kann nicht gestartet werden! Keine Internetverbindung
unsupported-hw-warning = Du versuchst, die Installation auf Hardware durchzuführen, die von der aktuellen ISO nicht unterstützt wird, daher kannst du für deine Installation keinen Support in Anspruch nehmen
desktop-on-handheld-error = Du versuchst, die Desktop-Edition auf einem Handheld-Gerät zu installieren. Bitte verwende die Handheld-Edition für ordnungsgemäße Unterstützung auf dieser Hardware
outdated-version-warning = Du nutzt eine alte Version von CachyOS, bitte nutze die aktuellste Version für Installationen
testing-iso-warning = Du verwendest eine alte Testing ISO, Testing-ISOs sind nicht stabil und nicht zum Verwenden geeignet
tweaksbrowser-label = Apps/Optimierungen
appbrowser-label = Apps installieren
troubleshooting-label = Problembehandlung
launch-start-label = Beim Systemstart ausführen
welcome-title = Willkommen bei CachyOS!
welcome-body =
    Danke, dass du dich unserer Community anschließt!

    Wir, die CachyOS-Entwickler, hoffen, dass du es genauso sehr genießen wirst, CachyOS zu benutzen, wie wir es genießen, es zu entwickeln. Die Links unten werden dir helfen dich in deinem neuen Betriebssystem zurechtzufinden. Genieße diese Erfahrung und zögere nicht dein Feedback an uns zu senden.
