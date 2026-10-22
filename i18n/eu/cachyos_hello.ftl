# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = CachyOS-en ongietorri pantaila

# Tweaks page
tweaks = Doikuntzak
fixes = Tresnak
applications = Aplikazioak
removed-db-lock = Pacman-en datu-basearen blokeoa kendu da!
lock-doesnt-exist = Pacman-en datu-basearen blokeoa ez da existitzen!
orphans-not-found = Ez da pakete umezurtzik aurkitu!
package-not-installed = '{$package_name}' paketea ez da instalatu!
gaming-package-installed = Jokoetarako paketeak instalatuta daude dagoeneko!
winboat-package-installed = Winboat paketeak instalatuta daude dagoeneko!
vram-management-package-installed = VRAM kudeaketarako paketeak instalatuta daude dagoeneko!

# Troubleshooting page
troubleshooting = Arazoen konponketa

# Dns Connections page
dns-settings = DNS ezarpenak
select-connection = Hautatu konexioa:
select-dns-server = Hautatu DNS zerbitzaria:
apply = Aplikatu
reset = Berrezarri
enable-encrypted-dns = Gaitu DNS {$protocol} bidez ({$abbr})
dns-type-label = DNS mota:
dns-type-tooltip = Aukeratu DNS kontsultak nola garraiatzen diren. Mota zifratuek (DoT/DoH/DoQ) zerbitzariaren euskarria behar dute; DoH eta DoQ aukerek blocky proxy lokala instalatu eta erabiltzen dute.
dns-type-plain = Arrunta (zifratu gabea)
dns-type-dot = DNS TLS bidez (DoT)
dns-type-doh = DNS HTTPS bidez (DoH)
dns-type-doq = DNS QUIC bidez (DoQ)
blocky-install-failed = Huts egin du blocky instalatzean {$mode} euskarrirako!
test-latency = Probatu hautatutako zerbitzariaren latentzia
test-latency-tooltip = Neurtu hautatutako DNS zerbitzariarekiko sareko latentzia
best-server = Hautatu latentzia onena duen zerbitzaria
best-server-tooltip = Probatu oinarrizko DNS zerbitzariak (iragazki-aldaerak kanpo utzita) eta hautatu azkarrena
latency-result = {""}
server-info = {""}
latency-testing = probatzen...
latency-timeout = denbora-muga gainditu da
latency-no-result = ez du zerbitzaririk erantzun
custom-dns = Pertsonalizatua
dhcp-automatic = DHCP (automatikoa)
custom-dns-ip = {$version} helbideak (komaz bereizita):
custom-dns-dot-hostname = DoT ostalari-izena (aukerakoa):
custom-dns-invalid = Sartu gutxienez IPv4 edo IPv6 helbide bat
custom-dns-invalid-hostname = DoT ostalari-izen baliogabea
custom-dns-doh-url = DoH URLa (DNS HTTPS bidez):
custom-dns-doh-url-required = Sartu https:// aurrizkiarekin hasten den DoH URL baliodun bat
custom-dns-doq-endpoint = DoQ amaiera-puntua (DNS QUIC bidez):
custom-dns-doq-endpoint-required = Sartu quic: edo quic:// aurrizkiarekin hasten den DoQ amaiera-puntu baliodun bat
dns-check-hint = Aplikatu ondoren, egiaztatu zure DNS hornitzailea hemen: {$dnscheck_url}
dns-server-changed = DNS zerbitzaria ongi aldatu da!
dns-server-pending = DNS zerbitzaria gorde da. Konektatu berriro sarea (edo desaktibatu eta aktibatu) aldaketak eragina izan dezan.
dns-server-failed = Huts egin du DNS zerbitzaria ezartzean!
dns-server-reset = DNS zerbitzaria berrezarri da!
dns-server-reset-failed = Huts egin du DNS zerbitzaria berrezartzean!
winboat-install-failed = Huts egin du Winboat instalatzean!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} gaituta
tweak-psd-tooltip = Erabili RAMa nabigatzailearen profiletarako (azkarragoa, diskoaren higadura txikiagoa)
tweak-oomd-tooltip = Prozesuak modu proaktiboan hil memoria gutxi dagoenean, izozteak saihesteko
tweak-bpftune-tooltip = Sistemaren sarea automatikoki sintonizatu
tweak-bluetooth-tooltip = Gaitu Bluetooth haririk gabeko gailuentzako euskarria (saguak, audioa, etab.)
tweak-ananicycpp-tooltip = Prozesuen lehentasunak automatikoki egokitu sistemaren erantzun hobea izateko
tweak-cachyupdate-tooltip = Eguneratzeen jakinarazlea erretiluan

# Tweaks page (fixes)
remove-lock-title = Kendu datu-basearen blokeoa
reinstall-title = Berrinstalatu pakete guztiak
reset-keyrings-title = Berrezarri gako-sortak (keyrings)
update-system-title = Sistemaren eguneratzea
remove-orphans-title = Kendu umezurtzak
clear-pkgcache-title = Garbitu paketeen cachea
rankmirrors-title = Ispiluen (mirrors) sailkapena
dnsserver-title = Aldatu DNS zerbitzaria
show-kwinw-debug-title = Erakutsi kwin(Wayland) debug-leihoa
install-gaming-title = Instalatu jokoetarako paketeak
install-winboat-title = Instalatu Winboat
install-vram-management-title = Instalatu VRAM kudeaketa
install-vram-management-tooltip = Lehenetsi VRAMa lehen planoko aplikaziorako, GPUaren kontrolatzaileak bufferrak sistemaren RAMera (GTT) isur ez ditzan.

# Main Page (buttons)
button-about-tooltip = Honi buruz
button-web-resource-tooltip = Web baliabidea
button-development-label = Garapena
button-software-label = Softwarea
button-donate-label = Dohaintza egin
button-forum-label = Foroa
button-installer-label = Abiarazi instalatzailea
button-involved-label = Parte hartu
button-readme-label = Irakurri
button-release-info-label = Bertsioaren informazioa
button-wiki-label = Wikia

# Main Page (sections)
section-docs = DOKUMENTAZIOA
section-installer = INSTALAZIOA
section-support = LAGUNTZA
section-project = PROIEKTUA

# Main Page (launch installer)
calamares-install-type = Calamares instalazio mota

# Main Page (body)
offline-error = Ezin da lineako instalazioa hasi! Ez dago interneteko konexiorik
unsupported-hw-warning = Uneko ISOak onartzen ez duen hardware batean instalatzen saiatzen ari zara, zure instalazioak ez du laguntza ofizialik jasoko
desktop-on-handheld-error = Mahaigaineko bertsioa gailu eramangarri (handheld) batean instalatzen saiatzen ari zara. Mesedez, erabili Handheld bertsioa hardware honetan euskarri egokia izateko
outdated-version-warning = CachyOS ISO bertsio zaharrago bat erabiltzen ari zara, mesedez, kontuan hartu azken bertsioa erabiltzea instalazioetarako
testing-iso-warning = Probetarako ISO bat erabiltzen ari zara; probetarako ISOak ez dira egonkorrak edo erabiltzeko prest daudenik kontsideratzen
tweaksbrowser-label = Aplikazioak/Doikuntzak
appbrowser-label = Instalatu aplikazioak
troubleshooting-label = Arazoen konponketa
launch-start-label = Abiarazi hasieran
welcome-title = Ongi etorri CachyOS-era!
welcome-body =
    Eskerrik asko gure komunitatean sartzeagatik!

    Guk, CachyOS-eko garatzaileok, CachyOS erabiltzen guk sortzen gozatu dugun adina gozatzea espero dugu. Azpiko estekek zure sistema eragile berriarekin lehen urratsak ematen lagunduko dizute. Beraz, gozatu esperientziaz, eta ez izan zalantzarik zure iritziak guri bidaltzeko.
