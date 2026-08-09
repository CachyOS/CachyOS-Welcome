# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Schermata di benvenuto per CachyOS

# Tweaks page
tweaks = Personalizzazioni
fixes = Utilità
applications = Applicazioni
removed-db-lock = Il blocco della base di dati di pacman è stato rimosso!
lock-doesnt-exist = Il blocco della base di dati di pacman non esiste!
orphans-not-found = Nessun pacchetto orfano trovato!
package-not-installed = Il pacchetto '{$package_name}' non è stato installato!
gaming-package-installed = I pacchetti Gaming sono già installati!
winboat-package-installed = I pacchetti Winboat sono già installati!
vram-management-package-installed = I pacchetti VRAM management sono già installati!


# Troubleshooting page
troubleshooting = Risoluzione dei problemi

# Dns Connections page
dns-settings = Impostazioni DNS
select-connection = Seleziona Connessione:
select-dns-server = Seleziona server DNS:
apply = Applica
reset = Reimposta
enable-encrypted-dns = Abilita DNS su {$protocol} ({$abbr})
blocky-install-failed = Errore durante l'installazione di blocky per il supporto {$mode}!
test-latency = Test latenza del server selezionato
test-latency-tooltip = Misura la latenza di rete verso il server DNS selezionato
best-server = Seleziona miglior server per latenza
best-server-tooltip = Testa i server DNS base (escludendo le varianti di filtraggio) e seleziona il più veloce
latency-result = {""}
server-info = {""}
latency-testing = test in corso...
latency-timeout = timeout
latency-no-result = nessun server ha risposto
custom-dns = Personalizzato
dhcp-automatic = DHCP (automatico)
custom-dns-ip = Indirizzi {$version} (separati da virgola):
custom-dns-dot-hostname = Hostname DoT (facoltativo):
custom-dns-invalid = Per favore inserisci almeno un indirizzo IPv4 o IPv6
custom-dns-invalid-hostname = Hostname DoT non valido
custom-dns-doh-url = Indirizzo DoH (per DNS su HTTPS):
custom-dns-doh-url-required = Per favore inserisci un indirizzo DoH valido che inizia con https://
custom-dns-doq-endpoint = Endpoint DoQ (per DNS su QUIC):
custom-dns-doq-endpoint-required = Per favore inserisci un endpoint DoQ valido che inizia con quic: o quic://
dns-check-hint = Dopo l'applicazione, verifica il tuo provider DNS su {$dnscheck_url}
dns-server-changed = Il server DNS è stato cambiato con successo!
dns-server-failed = Impostazione del server DNS non riuscita!
dns-server-reset = Il server DNS è stato reimpostato!
dns-server-reset-failed = Reimpostazione del server DNS non riuscita!
winboat-install-failed = Errore durante l'installazione di Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} abilitato
tweak-psd-tooltip = Utilizza la RAM per i profili del browser (più veloce, minore usura del disco)
tweak-oomd-tooltip = Termina proattivamente i processi in caso di memoria insufficiente per prevenire blocchi
tweak-bpftune-tooltip = Ottimizza automaticamente la rete di sistema
tweak-bluetooth-tooltip = Abilita il supporto per i dispositivi wireless Bluetooth (mouse, cuffie, ecc.)
tweak-ananicycpp-tooltip = Regola automaticamente le priorità dei processi per una migliore reattività del sistema
tweak-cachyupdate-tooltip = Notifica gli aggiornamenti nel vassoio di sistema

# Tweaks page (fixes)
remove-lock-title = Rimuovi il blocco del database
reinstall-title = Reinstalla tutti i pacchetti
reset-keyrings-title = Reimposta i portachiavi
update-system-title = Aggiorna il sistema
remove-orphans-title = Rimuovi gli orfani
clear-pkgcache-title = Pulisci la cache dei pacchetti
rankmirrors-title = Classifica i mirror
dnsserver-title = Cambia server DNS
show-kwinw-debug-title = Mostra la finestra di debug kwin (Wayland)
install-gaming-title = Installa i pacchetti Gaming
install-winboat-title = Installa Winboat
install-vram-management-title = Installa VRAM Management
install-vram-management-tooltip = Assegna priorità alla VRAM per l'applicazione in primo piano in modo che il driver della GPU eviti di riversare i buffer nella RAM di sistema (GTT).      

# Main Page (buttons)
button-about-tooltip = Informazioni
button-web-resource-tooltip = Risorse in rete
button-development-label = Sviluppo
button-software-label = Software
button-donate-label = Supportaci
button-forum-label = Forum
button-installer-label = Lancia l'installer
button-involved-label = Partecipa
button-readme-label = Leggimi
button-release-info-label = Note sulla versione
button-wiki-label = Wiki

# Main Page (sections)
section-docs = DOCUMENTAZIONE
section-installer = INSTALLAZIONE
section-support = SUPPORTO
section-project = PROGETTO

# Main Page (launch installer)
calamares-install-type = Tipo di installazione Calamares

# Main Page (body)
offline-error = Impossibile avviare l'installazione online! Connessione a internet assente
unsupported-hw-warning = Si sta tentando di effettuare l'installazione su un hardware non supportato dall'ISO corrente; l'installazione non potrà beneficiare dell'assistenza
desktop-on-handheld-error = Si sta tentando di installare l'edizione Desktop su un dispositivo portatile. Si prega di utilizzare l'edizione Handheld per un supporto adeguato su questo hardware
outdated-version-warning = Stai usando una versione obsoleta dell'ISO di CachyOS, considera di utilizzare l'ultima versione per le installazioni
testing-iso-warning = Stai usando una ISO di test, le ISO di test non sono considerate stabili e pronte per l'uso
tweaksbrowser-label = Applicazioni/Personalizzazioni
appbrowser-label = Installa Applicazioni
troubleshooting-label = Risoluzione dei problemi
launch-start-label = Lancia all'avvio
welcome-title = Benvenuto in CachyOS!
welcome-body =
    Grazie per esserti unito alla nostra comunità!

    Noi, gli sviluppatori di CachyOS, speriamo che trovi l'utilizzo di CachyOS altrettanto piacevole quanto lo troviamo noi nello svilupparlo. I collegamenti sottostanti ti aiuteranno a orientarti nel tuo nuovo sistema operativo. Per cui goditi l'esperienza e non esitare a inviarci la tua opinione.
