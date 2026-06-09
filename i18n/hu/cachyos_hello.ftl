# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Üdvözlőképernyő a CachyOS-hez

# Tweaks page
tweaks = Finomhangolások
fixes = Segédprogramok
applications = Alkalmazások
removed-db-lock = A Pacman adatbázis zárolása eltávolítva!
lock-doesnt-exist = A Pacman adatbázis zárolása nem létezik!
orphans-not-found = Nem találhatók árva csomagok!
package-not-installed = A(z) „{$package_name}” csomag nem lett telepítve!
gaming-package-installed = A játékcsomagok már telepítve vannak!
winboat-package-installed = A Winboat-csomagok már telepítve vannak!
vram-management-package-installed = A VRAM-kezelő csomagok már telepítve vannak!

# Application Browser page
advanced-btn = speciális
reset-btn = visszaállítás
update-system-app-btn = RENDSZER FRISSÍTÉSE
application-column = Alkalmazás
description-column = Leírás
install-remove-column = Telepítés/Eltávolítás
advanced-btn-tooltip = Kibővített csomagválasztás megjelenítése
reset-btn-tooltip = Jelenlegi kiválasztás visszaállítása...
update-system-app-btn-tooltip = A kiválasztott beállítások alkalmazása a rendszerre

# Dns Connections page
dns-settings = DNS-beállítások
select-connection = Kapcsolat kiválasztása:
select-dns-server = DNS-kiszolgáló kiválasztása:
apply = Alkalmazás
reset = Visszaállítás
enable-dot = DNS over TLS (DoT) engedélyezése
dot-tooltip = DNS-lekérdezések titkosítása TLS-sel a jobb adatvédelem érdekében (kiszolgáló támogatás szükséges)
enable-doh = DNS over HTTPS (DoH) engedélyezése
doh-tooltip = Titkosítja a DNS-lekérdezéseket HTTPS-en keresztül a helyi blocky-proxyn át (kiszolgálótámogatást igényel, telepíti a blocky-t)
doh-blocky-install-failed = Nem sikerült telepíteni a blocky-t a DoH-támogatáshoz!
test-latency = Kiválasztott kiszolgáló késleltetésének tesztelése
test-latency-tooltip = Hálózati késleltetés mérése a kiválasztott DNS-kiszolgálóhoz
best-server = Legjobb kiszolgáló kiválasztása késleltetés alapján
best-server-tooltip = Alap DNS-kiszolgálók tesztelése (szűrőváltozatok nélkül) és a leggyorsabb kiválasztása
latency-result = {""}
server-info = {""}
latency-testing = tesztelés...
latency-timeout = időtúllépés
latency-no-result = egyetlen kiszolgáló sem válaszolt
custom-dns = Egyéni
dhcp-automatic = DHCP (automatikus)
custom-dns-ipv4 = IPv4-címek (vesszővel elválasztva):
custom-dns-ipv6 = IPv6-címek (vesszővel elválasztva):
custom-dns-dot-hostname = DoT-gépnév (opcionális):
custom-dns-invalid = Adj meg legalább egy IPv4- vagy IPv6-címet!
custom-dns-invalid-hostname = Érvénytelen DoT-gépnév
custom-dns-doh-url = DoH-URL (a DNS over HTTPS-hez):
custom-dns-doh-url-required = Adj meg egy érvényes, https://-sel kezdődő DoH-URL-t!
dns-check-hint = Alkalmazás után ellenőrizheted a DNS-szolgáltatódat itt:
dns-server-changed = DNS-kiszolgáló sikeresen megváltozott!
dns-server-failed = Nem sikerült a DNS-kiszolgáló beállítása!
dns-server-reset = DNS-kiszolgáló visszaállítva!
dns-server-reset-failed = Nem sikerült a DNS-kiszolgáló visszaállítása!
winboat-install-failed = Nem sikerült a Winboat telepítése!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} engedélyezve
tweak-psd-tooltip = RAM használata a böngészőprofilokhoz (gyorsabb, kevesebb lemezhasználat)
tweak-oomd-tooltip = Folyamatok proaktív kilövése memóriahiány esetén a fagyások elkerülésére
tweak-bpftune-tooltip = A hálózati beállítások automatikus hangolása
tweak-bluetooth-tooltip = Bluetooth eszközök (egér, hang stb.) támogatásának engedélyezése
tweak-ananicycpp-tooltip = Folyamatprioritások automatikus beállítása a rendszer jobb válaszkészsége érdekében
tweak-cachyupdate-tooltip = Frissítésértesítő a tálcán

# Tweaks page (fixes)
remove-lock-title = Adatbáziszárolás eltávolítása
reinstall-title = Összes csomag újratelepítése
reset-keyrings-title = Kulcstartók visszaállítása
update-system-title = Rendszerfrissítés
remove-orphans-title = Árva csomagok eltávolítása
clear-pkgcache-title = Csomaggyorsítótár törlése
rankmirrors-title = Tükrök rangsorolása
dnsserver-title = DNS-kiszolgáló megváltoztatása
show-kwinw-debug-title = KWin (Wayland) hibakeresési ablak megjelenítése
install-gaming-title = Játékcsomagok telepítése
install-winboat-title = Winboat telepítése
install-vram-management-title = VRAM-kezelés telepítése
install-vram-management-tooltip = Előnyben részesíti a VRAM-ot az előtérben futó alkalmazásnál, így a GPU illesztőprogramja elkerüli a pufferek rendszermemóriába (GTT) való áthelyezését.

# Main Page (buttons)
button-about-tooltip = Névjegy
button-web-resource-tooltip = Webes forrás
button-development-label = Fejlesztés
button-software-label = Szoftver
button-donate-label = Adományozás
button-forum-label = Fórum
button-installer-label = Telepítő indítása
button-involved-label = Csatlakozz hozzánk
button-readme-label = Olvasd el
button-release-info-label = Kiadási információk
button-wiki-label = Wiki

# Main Page (sections)
section-docs = DOKUMENTÁCIÓ
section-installer = TELEPÍTÉS
section-support = TÁMOGATÁS
section-project = PROJEKT

# Main Page (launch installer)
recommended = ajánlott
calamares-install-type = Calamares telepítési típus

# Main Page (body)
offline-error = Nem sikerült elindítani az online telepítést! Nincs internetkapcsolat
unsupported-hw-warning = Olyan hardverre próbálsz telepíteni, amit ez az ISO nem támogat, így a telepítésed nem lesz jogosult hivatalos támogatásra
desktop-on-handheld-error = A Desktop kiadást próbálod telepíteni egy kézi eszközre. A megfelelő támogatás érdekében ezen a hardveren használd a Handheld kiadást
outdated-version-warning = Egy régebbi CachyOS ISO-t használsz, a telepítéshez érdemesebb a legújabb verziót használnod
testing-iso-warning = Ez egy tesztelésre szánt ISO, nem számít stabilnak és mindennapi használatra késznek
tweaksbrowser-label = Alkalmazások/Finomhangolások
appbrowser-label = Alkalmazások telepítése
launch-start-label = Automatikus indítás
welcome-title = Üdvözlünk a CachyOS-ben!
welcome-body =
    Köszönjük, hogy csatlakoztál a közösségünkhöz!

    Mi, a CachyOS fejlesztői reméljük, hogy legalább annyira élvezni fogod a CachyOS használatát, mint amennyire mi élveztük a megalkotását. Az alábbi linkek segítenek az elindulásban az új operációs rendszerrel. Jó szórakozást, és ne habozz visszajelzést küldeni nekünk!
