# About dialog
about-dialog-title = Benvinguda del CachyOS
about-dialog-comments = Pantalla de benvinguda per al CachyOS

# Tweaks page
tweaks = Retocs
fixes = Utilitats
applications = Aplicacions
removed-db-lock = El blocatge de la base de dades del Pacman s'ha suprimit!
lock-doesnt-exist = El blocatge de la base de dades del Pacman no existeix!
orphans-not-found = No s'ha trobat cap paquet orfe!
package-not-installed = El paquet {$package_name} no s'ha instal·lat!
gaming-package-installed = Els paquets de joc ja estan instal·lats!
winboat-package-installed = Els paquets del Winboat ja estan instal·lats!

# Application Browser page
advanced-btn = Avançat
reset-btn = Restabliment
update-system-app-btn = ACTUALITZACIÓ DEL SISTEMA
application-column = Aplicació
description-column = Descripció
install-remove-column = Instal·la / Suprimeix
advanced-btn-tooltip = Commuta a una selecció de paquets ampliada
reset-btn-tooltip = Restableix les seleccions actuals...
update-system-app-btn-tooltip = Aplica les seleccions actuals al sistema

# Dns Connections page
dns-settings = Paràmetres del DNS
select-connection = Seleccioneu la connexió:
select-dns-server = Seleccioneu el servidor de DNS:
apply = Apica-ho
reset = Restableix-ho
enable-dot = Activa DNS sobre TLS (DoT)
dot-tooltip = Encripta les consultes de DNS amb TLS per millorar la privadesa (requereix suport del servidor)
enable-doh = Habilita el DNS sobre HTTPS (DoH)
doh-tooltip = Encripta les consultes de DNS amb HTTPS a través d'un intermediari local amb blocatge (requereix compatibilitat amb el servidor, instal·la el Blocky)
doh-blocky-install-failed = Ha fallat instal·lar el Blocky per a la compatibilitat amb DoH!
test-latency = Prova la latència del servidor seleccionat
test-latency-tooltip = Mesura la latència de la xarxa al servidor DNS seleccionat
best-server = Selecciona el servidor millor segons la latència
best-server-tooltip = Prova els servidors DNS bàsics (sense variants de filtratge) i selecciona'n el més ràpid
latency-result = {""}
server-info = {""}
latency-testing = Es prova...
latency-timeout = Temps esgotat
latency-no-result = No ha respost cap servidor.
custom-dns = Personalitzat
custom-dns-ipv4 =  Adreces IPv4 (separades per comes):
custom-dns-ipv6 = Adreces IPv6 (separades per comes):
custom-dns-dot-hostname = Nom d'amfitrió del DoT (opcional):
custom-dns-invalid = Si us plau, introduïu com a mínim una adreça IPv4 o IPv6.
custom-dns-invalid-hostname = Nom d'amfitrió DoT no vàlid.
custom-dns-doh-url = URL de DoH (per a DNS sobre HTTPS):
custom-dns-doh-url-required = Introduïu un URL vàlid del DoH que comenci amb https://
dns-check-hint = Després d'aplicar-ho, verifiqueu el proveïdor de DNS a
dns-server-changed = S'ha canviat correctament el servidor de DNS!
dns-server-failed = Ha fallat establir el servidor de DNS!
dns-server-reset = S'ha restablert el servidor de DNS!
dns-server-reset-failed = Ha fallat restablir el servidor de DNS!
winboat-install-failed = Ha fallat instal·lar el Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} habilitat
tweak-psd-tooltip = Usa la memòria RAM per als perfils del navegador (més ràpid, menys desgast del disc)
tweak-oomd-tooltip = Suprimeix proactivament els processos durant la memòria baixa per evitar blocatges
tweak-bpftune-tooltip = Ajusta automàticament la xarxa del sistema
tweak-bluetooth-tooltip = Habilita la compatibilitat amb dispositius sense fil Bluetooth (ratolins, àudio, etc.)
tweak-ananicycpp-tooltip = Ajusta automàticament les prioritats dels processos per a una resposta del sistema millor
tweak-cachyupdate-tooltip = Actualitza el notificador de la safata

# Tweaks page (fixes)
remove-lock-title = Suprimeix el blocatge de la base de dades
reinstall-title = Reinstal·la tots els paquets
reset-keyrings-title = Restableix els clauers
update-system-title = Actualització del sistema
remove-orphans-title = Suprimeix els paquets orfes
clear-pkgcache-title = Neteja la cau de paquets
rankmirrors-title = Classifica les rèpliques
dnsserver-title = Canvia el servidor de DNS
show-kwinw-debug-title = Mostra la finestra de depuració del kwin (Wayland)
install-gaming-title = Instal·la paquets de jocs
install-winboat-title = Instal·la el Winboat

# Main Page (buttons)
button-about-tooltip = Quant a
button-web-resource-tooltip = Recurs web
button-development-label = Desenvolupament
button-software-label = Programari
button-donate-label = Feu una donació
button-forum-label = Fòrum
button-installer-label = Inicia'n l'instal·lador
button-involved-label = Col·laboreu-hi
button-readme-label = Llegiu-me
button-release-info-label = Informació de la versió
button-wiki-label = Wiki

# Main Page (sections)
section-docs = DOCUMENTACIÓ
section-installer = INSTAL·LACIÓ
section-support = SUPORT
section-project = PROJECTE

# Main Page (launch installer)
recommended = recomanat
calamares-install-type = Tipus d'instal·lació per al Calamares

# Main Page (body)
offline-error = No es pot iniciar la instal·lació en línia! No hi ha connexió a Internet.
unsupported-hw-warning = Esteu intentant d'instal·lar el sistema en un maquinari que no és compatible amb la imatge ISO actual. La instal·lació no podrà rebre suport.
desktop-on-handheld-error = Esteu intentant d'instal·lar l'edició d'escriptori en un dispositiu portàtil. Si us plau, useu l'edició Handheld per a un suport adequat en aquest maquinari.
outdated-version-warning = Useu una imatge més antiga del CachyOS. Si us plau, considereu usar-ne la versió més recent per a les instal·lacions.
testing-iso-warning = Useu una imatge ISO de prova. Les ISO de prova no es consideren estables i llestes per a un ús productiu.
tweaksbrowser-label = Aplicacions / Retocs
appbrowser-label = Instal·leu apliacions
launch-start-label = Obre-ho a l'inici
welcome-title = Us donem la benvinguda al CachyOS!
welcome-body =
    Moltes mercès per unir-vos a la nostra comunitat!

    Nosaltres, els desenvolupadors del CachyOS, esperem que gaudiu d'usar el CachyOS tant com a nosaltres ens agrada fer-lo. Els enllaços següents us ajudaran a començar. Gaudiu de l'experiència i no dubteu a enviar-nos-en comentaris.
