# Boite de dialogue : À propos
about-dialog-title = CachyOS Hello
about-dialog-comments = Écran de bienvenue pour CachyOS

# Page des modifications
tweaks = Modifications
fixes = Utilitaires
applications = Applications
removed-db-lock = Le verrou de la base de données Pacman a été supprimé !
lock-doesnt-exist = Le verrou de la base de données Pacman n'existe pas !
orphans-not-found = Aucun paquet orphelin trouvé !
package-not-installed = Le paquet '{$package_name}' n'a pas été installé !
gaming-package-installed = Les paquets de jeux sont déjà installés !
winboat-package-installed = Les paquets Winboat sont déjà installés !
vram-management-package-installed = Les paquets de gestion de la mémoire vidéo (VRAM) sont déjà installés !

# Page du navigateur d'applications

# Page de dépannage
troubleshooting = Dépannage

# Page « Connexions DNS »
dns-settings = Paramètres DNS
select-connection = Sélectionner une connexion :
select-dns-server = Selectionner un serveur DNS :
apply = Appliquer
reset = Réinitialiser
enable-encrypted-dns = Activer le DNS sur {$protocol} ({$abbr})
blocky-install-failed = Échec de l'installation de blocky pour le support {$mode} !
test-latency = Tester la latence du serveur sélectionné
test-latency-tooltip = Mesurer la latence réseau vers le serveur DNS sélectionné
best-server = Sélectionner le meilleur serveur par latence
best-server-tooltip = Tester les serveurs DNS de base (sans variantes de filtrage) et sélectionner le plus rapide
latency-result = {""}
server-info = {""}
latency-testing = test en cours...
latency-timeout = délai dépassé
latency-no-result = aucun serveur n'a répondu
custom-dns = Personnalisé
dhcp-automatic = DHCP (automatique)
custom-dns-ip = Adresses {$version} (séparées par des virgules) :
custom-dns-dot-hostname = Nom d'hôte DoT (optionnel) :
custom-dns-invalid = Veuillez entrer au moins une adresse IPv4 ou IPv6
custom-dns-invalid-hostname = Nom d'hôte DoT invalide
custom-dns-doh-url = URL DoH (pour DNS sur HTTPS) :
custom-dns-doh-url-required = Veuillez entrer une URL DoH valide commençant par https://
custom-dns-doq-endpoint = Point de terminaison DoQ (pour DNS sur QUIC) :
custom-dns-doq-endpoint-required = Veuillez saisir un point de terminaison DoQ valide commençant par quic: ou quic://
dns-check-hint = Après application, vérifiez votre fournisseur DNS sur {$dnscheck_url}
dns-server-changed = Le serveur DNS a été modifié avec succès !
dns-server-failed = Échec de la modification du serveur DNS !
dns-server-reset = Le serveur DNS a été réinitialisé !
dns-server-reset-failed = Échec de la réinitialisation du serveur DNS !
winboat-install-failed = Échec de l'installation de Winboat !

# Page des modifications (modifications)
tweak-enabled-title = {$tweak} activé
tweak-psd-tooltip = Utiliser la mémoire vive (RAM) pour les profils du navigateur (plus rapide, moins d'usure du disque)
tweak-oomd-tooltip = Tuer proactivement les processus en cas de mémoire insuffisante pour éviter les blocages
tweak-bpftune-tooltip = Régler automatiquement le réseau système
tweak-bluetooth-tooltip = Activer la prise en charge des appareils sans fil Bluetooth (souris, audio, etc.)
tweak-ananicycpp-tooltip = Ajuster automatiquement la priorité des processus pour améliorer la réactivité du système
tweak-cachyupdate-tooltip = Notifie les mises à jour dans la zone de notification

# Page des modifications (corrections)
remove-lock-title = Supprimer le verrou de la base de données
reinstall-title = Réinstaller tous les paquets
reset-keyrings-title = Réinitialiser les trousseaux de clés
update-system-title = Mise à jour du système
remove-orphans-title = Supprimer les orphelins
clear-pkgcache-title = Vider le cache des paquets
rankmirrors-title = Classer les miroirs
dnsserver-title = Changer le serveur DNS
show-kwinw-debug-title = Afficher la fenêtre de débogage de kwin(Wayland)
install-gaming-title = Installer les paquets de jeux
install-winboat-title = Installer Winboat
install-vram-management-title = Installer la gestion VRAM
install-vram-management-tooltip = Prioriser la mémoire vidéo (VRAM) pour l'application au premier plan afin d'éviter que le pilote GPU ne déverser les tampons dans la mémoire système (GTT).

# Page principale (boutons)
button-about-tooltip = À propos
button-web-resource-tooltip = Ressource en ligne
button-development-label = Développement
button-software-label = Logiciel
button-donate-label = Faire un don
button-forum-label = Forum
button-installer-label = Lancer l'installateur
button-involved-label = S'impliquer
button-readme-label = Lisez-moi
button-release-info-label = Informations sur la version
button-wiki-label = Wiki

# Page principale (sections)
section-docs = DOCUMENTATION
section-installer = INSTALLATION
section-support = SUPPORT
section-project = PROJET

# Page principale (programme d'installation)
calamares-install-type = Type d'installion Calamares

# Page principale (corps)
offline-error = Impossible de démarrer l'installation en ligne ! Pas de connexion internet
unsupported-hw-warning = Vous tentez d'installer sur du matériel non supporté par l\'ISO actuelle, votre installation ne sera pas éligible à de l\'assistance
desktop-on-handheld-error = Vous tentez d\'installer l\'édition Desktop sur un appareil portable. Veuillez utiliser l\'édition Handheld pour une prise en charge correcte de ce matériel
outdated-version-warning = Vous utilisez une ancienne version de l'ISO de CachyOS, veuillez utiliser la dernière version pour vos installations
testing-iso-warning = Vous utilisez une ISO de test, les ISOs de test ne sont pas considérées comme stables ni prêtes à l'emploi.
tweaksbrowser-label = Applications/Modifications
appbrowser-label = Installer des Applications
launch-start-label = Lancer au démarrage
welcome-title = Bienvenue sur CachyOS !
welcome-body =
    Merci de rejoindre notre communauté !

    Nous, les développeurs de CachyOS, espérons que vous aimerez autant CachyOS que nous avons aimé à le développer. Les liens ci-dessous vous aiderons à prendre en main votre nouveau système d\'exploitation. Profitez donc de l\'expérience, et n'hésitez pas à nous envoyer vos retours.
