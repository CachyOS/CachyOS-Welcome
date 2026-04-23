# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Начален екран за CachyOS

# Tweaks page
tweaks = Настройки
fixes = Помощни програми
applications = Приложения
removed-db-lock = Заключването на Pacman-базата данни беше премахнато!
lock-doesnt-exist = Заключването на Pacman-базата данни не е намерено!
orphans-not-found = Не са намерени осиротели пакети!
package-not-installed = Пакетът '{$package_name}' не е инсталиран!
gaming-package-installed = Пакетите за гейминг вече са инсталирани!
winboat-package-installed = Winboat пакетите вече са инсталирани!
vram-management-package-installed = VRAM management пакетите вече са инсталирани!!

# Application Browser page
advanced-btn = Разширени
reset-btn = Нулиране
update-system-app-btn = АКТУАЛИЗИРАНЕ НА СИСТЕМАТА
application-column = Приложение
description-column = Описание
install-remove-column = Инсталиране/Премахване
advanced-btn-tooltip = Превключва към разширен списък с пакети
reset-btn-tooltip = Нулиране на текущия ви избор...
update-system-app-btn-tooltip = Прилагане на текущия ви избор към системата

# Dns Connections page
dns-settings = DNS настройки
select-connection = Изберете връзка:
select-dns-server = Изберете DNS сървър:
apply = Прилагане
reset = Нулиране
enable-dot = Активиране на криптиране на DNS с TLS
dot-tooltip = Криптира DNS заявки чрез TLS за подобрена поверителност (изисква поддръжка от сървъра)
enable-doh = Активиране на криптиране на DNS с HTTPS (DoH)
doh-tooltip = Криптира DNS заявки чрез HTTPS чрез локален прокси blocky (изисква поддръжка от сървъра, инсталира blocky)
doh-blocky-install-failed = Неуспешно инсталиране на blocky за поддръжка на DoH!
test-latency = Тест на забавяне към избрания сървър
test-latency-tooltip = Измерване на мрежовото забавяне до избрания DNS сървър
best-server = Изберете най-добър сървър според забавянето
best-server-tooltip = Тестване на стандартните DNS сървъри (без тези с филтрация) и избор на най-бързия
latency-result = {""}
server-info = {""}
latency-testing = тестване...
latency-timeout = изтекло време
latency-no-result = няма отговор от сървъра
custom-dns = Персонализиран
dhcp-automatic = DHCP (автоматично)
custom-dns-ipv4 = IPv4 адреси (разделени със запетая):
custom-dns-ipv6 = IPv6 адреси (разделени със запетая):
custom-dns-dot-hostname = DoT име на хост (по избор):
custom-dns-invalid = Моля, въведете поне един IPv4 или IPv6 адрес
custom-dns-invalid-hostname = Невалидно DoT име на хост
custom-dns-doh-url = DoH URL (за DNS over HTTPS):
custom-dns-doh-url-required = Моля, въведете валиден DoH URL, започващ с https://
dns-check-hint = След прилагане, проверете вашия DNS доставчик на
dns-server-changed = DNS сървърът беше успешно променен!
dns-server-failed = Неуспешно задаване на DNS сървър!
dns-server-reset = DNS сървърът е нулиран!
dns-server-reset-failed = Неуспешно нулиране на DNS сървъра!
winboat-install-failed = Неуспешно инсталиране на Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} активиран
tweak-psd-tooltip = Съхраняване на профилите на браузъра в RAM (по-бързо, намалява износването на диска)
tweak-oomd-tooltip = Проактивно прекратяване на процеси при ниска памет за предотвратяване на замръзвания
tweak-bpftune-tooltip = Автоматична настройка на системната мрежа
tweak-bluetooth-tooltip = Активиране на поддръжка за Bluetooth устройства (мишки, аудио и др.)
tweak-ananicycpp-tooltip = Автоматично регулира приоритетите на процесите за по-добра отзивчивост на системата
tweak-cachyupdate-tooltip = Известие за обновления в системния трей

# Tweaks page (fixes)
remove-lock-title = Премахване на заключването на Pacman-базата данни
reinstall-title = Преинсталиране на всички пакети
reset-keyrings-title = Нулиране на Pacman ключодържателите
update-system-title = Актуализация на системата
remove-orphans-title = Премахване на осиротели пакети
clear-pkgcache-title = Изчистване на пакетния кеш
rankmirrors-title = Класиране на mirror сървъри
dnsserver-title = Промяна на DNS сървъра
show-kwinw-debug-title = Отваряне на прозореца за отстраняване на грешки на kwin (Wayland)
install-gaming-title = Инсталиране на пакети за гейминг
install-winboat-title = Инсталиране на Winboat
install-vram-management-title = Инсталиране на VRAM Management
install-vram-management-tooltip = Приоритизирайте VRAM за приложението на преден план, така че драйверът на GPU да избягва изхвърлянето на буфери в системната RAM (GTT).

# Main Page (buttons)
button-about-tooltip = Относно
button-web-resource-tooltip = Уеб ресурс
button-development-label = Разработка
button-software-label = Софтуер
button-donate-label = Подкрепи ни
button-forum-label = Форум
button-installer-label = Стартиране на инсталатора
button-involved-label = Включете се
button-readme-label = Прочети ме
button-release-info-label = Списък на промените
button-wiki-label = Уики

# Main Page (sections)
section-docs = ДОКУМЕНТАЦИЯ
section-installer = ИНСТАЛАЦИЯ
section-support = ПОДДРЪЖКА
section-project = ПРОЕКТ

# Main Page (launch installer)
recommended = препоръчително
calamares-install-type = Тип инсталация Calamares

# Main Page (body)
offline-error = Не може да се стартира онлайн инсталацията! Няма интернет връзка.
unsupported-hw-warning = Опитвате се да инсталирате на хардуер, който не се поддържа от текущия ISO образ. Вашата инсталация няма да бъде поддържана.
desktop-on-handheld-error = Опитвате се да инсталирате Desktop версия на Handheld устройство. Този хардуер не се поддържа от текущия ISO образ. Моля, използвайте Handheld версията за пълна поддръжка.
outdated-version-warning = Използвате по-стара версия на CachyOS ISO. Моля, обмислете използването на най-новата версия за инсталации.
testing-iso-warning = Използвате тестов ISO образ. Тестовите ISO образи не се считат за стабилни и готови за употреба.
tweaksbrowser-label = Приложения/Настройки
appbrowser-label = Инсталиране на приложения
launch-start-label = Стартиране при вход
welcome-title = Добре дошли в CachyOS!
welcome-body =
    Благодарим ви, че се присъединихте към нашата общност!

    Надяваме се, че ще използвате CachyOS с удоволствие – точно толкова, колкото на нас ни доставя удоволствие да я създаваме. Връзките по-долу ще ви помогнат да започнете работа с новата си операционна система. Насладете се на работата с нея и не се притеснявайте да споделите мнението си.
