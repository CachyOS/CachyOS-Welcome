# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Pantalla de bienvenida de CachyOS

# Tweaks page
tweaks = Ajustes
fixes = Utilidades
applications = Aplicaciones
removed-db-lock = ¡Se eliminó el bloqueo de la base de datos de Pacman!
lock-doesnt-exist = ¡No existe un bloqueo de la base de datos de Pacman!
orphans-not-found = ¡No se encontraron paquetes huérfanos!
package-not-installed = ¡El paquete '{$package_name}' no se ha instalado!
gaming-package-installed = ¡Los paquetes de Gaming ya están instalados!
winboat-package-installed = ¡Los paquetes de Winboat ya están instalados!

# Application Browser page
advanced-btn = avanzado
reset-btn = restablecer
update-system-app-btn = ACTUALIZAR SISTEMA
application-column = Aplicación
description-column = Descripción
install-remove-column = Instalar/Eliminar
advanced-btn-tooltip = Alternar una selección ampliada de paquetes
reset-btn-tooltip = Restablecer las selecciones actuales...
update-system-app-btn-tooltip = Aplicar las selecciones actuales al sistema

# Dns Connections page
dns-settings = Ajustes de DNS
select-connection = Seleccionar conexión:
select-dns-server = Seleccionar servidor DNS:
apply = Aplicar
reset = Restablecer
enable-dot = Activar DNS sobre TLS (DoT)
dot-tooltip = Cifrar consultas DNS con TLS para mayor privacidad (requiere soporte del servidor)
enable-doh = Activar DNS sobre HTTPS (DoH)
doh-tooltip = Cifrar consultas DNS usando HTTPS mediante un proxy local Blocky (requiere soporte del servidor, instala Blocky)
doh-blocky-install-failed = ¡Error al instalar Blocky para el soporte de DoH!
test-latency = Probar latencia del servidor seleccionado
test-latency-tooltip = Medir la latencia de red hacia el servidor DNS seleccionado
best-server = Seleccionar el mejor servidor por latencia
best-server-tooltip = Probar servidores DNS base (sin variantes de filtrado) y seleccionar el más rápido
latency-result = {""}
server-info = {""}
latency-testing = probando...
latency-timeout = tiempo de espera agotado
latency-no-result = ningún servidor respondió
custom-dns = Personalizado
custom-dns-ipv4 = Direcciones IPv4 (separadas por comas):
custom-dns-ipv6 = Direcciones IPv6 (separadas por comas):
custom-dns-dot-hostname = Nombre de host DoT (opcional):
custom-dns-invalid = Por favor, introduzca al menos una dirección IPv4 o IPv6
custom-dns-invalid-hostname = Nombre de host DoT no válido
custom-dns-doh-url = URL de DoH (para DNS sobre HTTPS):
custom-dns-doh-url-required = Por favor, introduzca una URL de DoH válida que comience con https://
dns-check-hint = Después de aplicar, verifique su proveedor de DNS en
dns-server-changed = ¡El servidor DNS se cambió con éxito!
dns-server-failed = ¡Error al configurar el servidor DNS!
dns-server-reset = ¡El servidor DNS ha sido restablecido!
dns-server-reset-failed = ¡No se pudo restablecer el servidor DNS!
winboat-install-failed = ¡Error al instalar Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} habilitado
tweak-psd-tooltip = Usar RAM para perfiles del navegador (más rápido, menos desgaste del disco)
tweak-oomd-tooltip = Finalizar procesos proactivamente cuando haya poca memoria para evitar bloqueos
tweak-bpftune-tooltip = Ajustar automáticamente la red del sistema
tweak-bluetooth-tooltip = Activar soporte para dispositivos inalámbricos Bluetooth (ratón, audio, etc.)
tweak-ananicycpp-tooltip = Ajustar automáticamente las prioridades de procesos para una mejor respuesta del sistema
tweak-cachyupdate-tooltip = Notificador de actualizaciones en la bandeja

# Tweaks page (fixes)
remove-lock-title = Eliminar bloqueo de base de datos
reinstall-title = Reinstalar todos los paquetes
reset-keyrings-title = Restablecer depósitos de llaves (keyrings)
update-system-title = Actualización del sistema
remove-orphans-title = Eliminar paquetes huérfanos
clear-pkgcache-title = Limpiar caché de paquetes
rankmirrors-title = Evaluar espejos (mirrors)
dnsserver-title = Cambiar servidor DNS
show-kwinw-debug-title = Mostrar ventana de depuración de KWin (Wayland)
install-gaming-title = Instalar paquetes de Gaming
install-winboat-title = Instalar Winboat

# Main Page (buttons)
button-about-tooltip = Acerca de
button-web-resource-tooltip = Recurso web
button-development-label = Desarrollo
button-software-label = Software
button-donate-label = Donar
button-forum-label = Foro
button-installer-label = Ejecutar instalador
button-involved-label = Participar
button-readme-label = Léame
button-release-info-label = Info de versión
button-wiki-label = Wiki

# Main Page (sections)
section-docs = DOCUMENTACIÓN
section-installer = INSTALACIÓN
section-support = SOPORTE
section-project = PROYECTO

# Main Page (launch installer)
recommended = recomendado
calamares-install-type = Tipo de instalación Calamares

# Main Page (body)
offline-error = ¡No se pudo iniciar la instalación en línea! No hay conexión a internet
unsupported-hw-warning = Se está intentando instalar en hardware no soportado por la ISO actual; la instalación no contará con soporte oficial.
desktop-on-handheld-error = Se está intentando instalar la edición de Escritorio en un dispositivo portátil. Por favor, utilice la edición Handheld para obtener un soporte adecuado en este hardware.
outdated-version-warning = Se está utilizando una versión antigua de la ISO de CachyOS; por favor, considere usar la última versión para realizar la instalación.
testing-iso-warning = Se está utilizando una ISO de prueba; estas versiones no se consideran estables ni listas para su uso general.
tweaksbrowser-label = Aplicaciones/Ajustes
appbrowser-label = Instalar Aplicaciones
launch-start-label = Ejecutar al inicio
welcome-title = ¡Bienvenido a CachyOS!
welcome-body =
    ¡Gracias por unirse a nuestra comunidad!

    Quienes desarrollamos CachyOS deseamos que disfrute este sistema operativo tanto como nosotros disfrutamos creándolo. Los enlaces a continuación le ayudarán a dar sus primeros pasos. Disfrute la experiencia y no dude en enviarnos sus comentarios.
