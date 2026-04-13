# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = CachyOS pasveikinimo langas

# Tweaks page
tweaks = Tinkinimas
fixes = Priemonės
applications = Programos
removed-db-lock = Pacman duomenų bazės užraktas pašalintas!
lock-doesnt-exist = Pacman duomenų bazės užrakto nėra!
orphans-not-found = Nereikalingų paketų nerasta!
package-not-installed = Paketas '{$package_name}' nėra įdiegtas!
gaming-package-installed = Žaidimams skirti paketai jau įdiegti!
winboat-package-installed = Winboat paketai jau įdiegti!
gpu-boosters-package-installed = GPU spartinimo paketai jau įdiegti!

# Application Browser page
advanced-btn = išplėstinis
reset-btn = atstatyti
update-system-app-btn = ATNAUJINTI SISTEMĄ
application-column = Programa
description-column = Aprašas
install-remove-column = Įdiegti/Šalinti
advanced-btn-tooltip = Perjungti išplėstinį paketų pasirinkimą
reset-btn-tooltip = Atstatyti dabartinius pasirinkimus...
update-system-app-btn-tooltip = Pritaikyti dabartinius pasirinkimus sistemai

# Dns Connections page
dns-settings = DNS nuostatos
select-connection = Pasirinkite ryšį:
select-dns-server = Pasirinkite DNS serverį:
apply = Taikyti
reset = Atstatyti
enable-dot = Įjungti DNS per TLS (DoT)
dot-tooltip = Šifruoti DNS užklausas naudojant TLS, kad būtų užtikrintas didesnis privatumas (reikalingas serverio palaikymas)
enable-doh = Įjungti DNS per HTTPS (DoH)
doh-tooltip = Šifruoti DNS užklausas naudojant HTTPS per blocky vietin tarpinį serverį (reikalingas serverio palaikymas, įdiegiamas blocky)
doh-blocky-install-failed = Nepavyko įdiegti blocky, reikalingo DoH palaikymui!
test-latency = Išmatuoti pasirinkto serverio delsą
test-latency-tooltip = Matuoti tinklo delsą iki pasirinkto DNS serverio
best-server = Parinkti geriausią serverį pagal delsą
best-server-tooltip = Patikrinti bazinius DNS serverius (neįtraukiant filtravimo variantų) ir parinkti greičiausią
latency-result = {""}
server-info = {""}
latency-testing = matuojama...
latency-timeout = skirtasis laikas
latency-no-result = neatsakė nė vienas serveris
custom-dns = Tinkintas
dhcp-automatic = DHCP (automatinis)
custom-dns-ipv4 = IPv4 adresai (atskirti kableliais):
custom-dns-ipv6 = IPv6 adresai (atskirti kableliais):
custom-dns-dot-hostname = DoT serverio vardas (nebūtina):
custom-dns-invalid = Įveskite bent vieną IPv4 arba IPv6 adresą
custom-dns-invalid-hostname = Neteisingas DoT serverio vardas
custom-dns-doh-url = DoH URL (DNS per HTTPS):
custom-dns-doh-url-required = Įveskite tinkamą DoH URL, prasidedantį https://
dns-check-hint = Pritaikę pakeitimus, patikrinkite DNS teikėją čia:
dns-server-changed = DNS serveris sėkmingai pakeistas!
dns-server-failed = Nepavyko nustatyti DNS serverio!
dns-server-reset = DNS serverio nuostatos atstatytos!
dns-server-reset-failed = Nepavyko atstatyti DNS serverio nuostatų!
winboat-install-failed = Nepavyko įdiegti Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = Įjungta: {$tweak}
tweak-psd-tooltip = Naudoti RAM naršyklės profiliams (greičiau, mažiau dėvisi diskas)
tweak-oomd-tooltip = Esant mažai atminties, iš anksto užbaigti procesus, kad sistema neužstrigtų
tweak-bpftune-tooltip = Automatiškai derinti sistemos tinklą
tweak-bluetooth-tooltip = Įjungti Bluetooth belaidžių įrenginių palaikymą (pelės, garsas ir kt.)
tweak-ananicycpp-tooltip = Automatiškai koreguoti procesų prioritetus, kad sistema sparčiau reaguotų
tweak-cachyupdate-tooltip = Atnaujinimų pranešiklis sistemos dėkle

# Tweaks page (fixes)
remove-lock-title = Pašalinti duomenų bazės užraktą
reinstall-title = Perdiegti visus paketus
reset-keyrings-title = Atstatyti raktų rinkinius
update-system-title = Sistemos naujinimas
remove-orphans-title = Pašalinti nebereikalingus paketus
clear-pkgcache-title = Išvalyti paketų podėlį
rankmirrors-title = Surikiuoti veidrodžius
dnsserver-title = Pakeisti DNS serverį
show-kwinw-debug-title = Rodyti KWin (Wayland) derinimo langą
install-gaming-title = Įdiegti žaidimams skirtus paketus
install-winboat-title = Įdiegti Winboat
install-gpu-boosters-title = Įdiegti GPU spartinimo paketus
install-gpu-boosters-tooltip = Įdiegti dmemcg-booster ir plasma-foreground-booster, skirtus AMD arba Intel GPU

# Main Page (buttons)
button-about-tooltip = Apie
button-web-resource-tooltip = Žiniatinklio išteklius
button-development-label = Kūrimas
button-software-label = Programinė įranga
button-donate-label = Paremti
button-forum-label = Forumas
button-installer-label = Paleisti diegyklę
button-involved-label = Prisidėti
button-readme-label = README
button-release-info-label = Laidos informacija
button-wiki-label = Vikis

# Main Page (sections)
section-docs = DOKUMENTACIJA
section-installer = DIEGIMAS
section-support = PALAIKYMAS
section-project = PROJEKTAS

# Main Page (launch installer)
recommended = rekomenduojama
calamares-install-type = Calamares diegimo tipas

# Main Page (body)
offline-error = Nepavyko paleisti internetinio diegimo! Nėra interneto ryšio
unsupported-hw-warning = Bandote diegti įrangai, kurios ši ISO versija nepalaiko; tokiam diegimui palaikymas nebus teikiamas
desktop-on-handheld-error = Bandote diegti Desktop leidimą į delninį įrenginį. Kad ši įranga būtų tinkamai palaikoma, naudokite Handheld leidimą
outdated-version-warning = Naudojate senesnę CachyOS ISO versiją; diegdami apsvarstykite galimybę naudoti naujausią versiją
testing-iso-warning = Naudojate bandomąją ISO versiją; bandomosios ISO versijos nelaikomos stabiliomis ir parengtomis naudoti
tweaksbrowser-label = Programos / tinkinimas
appbrowser-label = Įdiegti programas
launch-start-label = Paleisti kartu su sistema
welcome-title = Sveiki atvykę į CachyOS!
welcome-body =
    Ačiū, kad prisijungėte prie mūsų bendruomenės!

    Mes, CachyOS kūrėjai, tikimės, kad naudotis CachyOS jums bus taip pat malonu, kaip mums ją kurti. Toliau pateiktos nuorodos padės pradėti darbą su naująja operacine sistema. Tad mėgaukitės patirtimi ir nedvejodami atsiųskite mums savo atsiliepimus.
