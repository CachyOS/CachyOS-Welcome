# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Приветствен екран за CachyOS

# Tweaks page
tweaks = Настройки
fixes = Помощни програми
applications = Приложения
removed-db-lock = Блокировката на базата данни на Pacman беше премахната!
lock-doesnt-exist = Блокировка на базата данни на Pacman не съществува!
orphans-not-found = Не бяха открити изоставени пакети!
package-not-installed = Пакетът '{$package_name}' не е инсталиран!
gaming-package-installed = Гейминг пакетите вече са инсталирани!
winboat-package-installed = Winboat пакетите вече са инсталирани!
vram-management-package-installed = Пакетите за управление на VRAM вече са инсталирани!


# Страница за Отстраняване на проблеми
troubleshooting = Отстраняване на проблеми
clear-pkgcache-title = Изчистване на кеша на пакетите
remove-lock-title = Премахване на заключването на базата Pacman
reset-keyrings-title = Нулиране на ключодържателите
reinstall-title = Преинсталиране на всички пакети
show-kwinw-debug-title = Показване на дебъг прозореца на kwin (Wayland)

# Dns Connections page
dns-settings = DNS настройки
select-connection = Изберете връзка:
select-dns-server = Изберете DNS сървър:
apply = Прилагане
reset = Нулиране
enable-encrypted-dns = Активиране на криптиране на DNS с {$protocol} ({$abbr})
dns-type-label = Тип DNS:
dns-type-tooltip =  Изберете как да се пренасят DNS заявките. Шифрованите типове (DoT/DoH/DoQ) изискват поддръжка от сървъра; DoH и DoQ инсталират и използват локален прокси blocky.
dns-type-plain = Обикновен (нешифрован)
dns-type-dot = DNS през TLS (DoT)
dns-type-doh = DNS през HTTPS (DoH)
dns-type-doq = DNS през QUIC (DoQ)
blocky-install-failed = Неуспешно инсталиране на blocky за поддръжка на {$mode}!
test-latency = Тест на латентност на избрания сървър
test-latency-tooltip = Измерване на латентността до избрания DNS сървър
best-server = Избор на най-бърз сървър по латентност
best-server-tooltip = Тестване на базовите DNS сървъри (без филтриращите варианти) и избор на най-бързия
latency-result =
server-info =
latency-testing = тестване...
latency-timeout = изтекло време
latency-no-result = няма отговор от сървъра
custom-dns = Персонализиран
dhcp-automatic = DHCP (автоматично)
custom-dns-ip = {$version} адреси (разделени със запетая):
custom-dns-dot-hostname = DoT име на хост (по избор):
custom-dns-invalid = Моля, въведете поне един IPv4 или IPv6 адрес
custom-dns-invalid-hostname = Невалидно DoT име на хост
custom-dns-doh-url = DoH URL (за DNS over HTTPS):
custom-dns-doh-url-required = Моля, въведете валиден DoH URL, започващ с https://
dns-check-hint = След прилагане, проверете вашия DNS доставчик на {$dnscheck_url}
dns-server-changed = DNS сървърът беше успешно променен!
dns-server-failed = Неуспешна настройка на DNS сървър!
dns-server-reset = DNS сървърът е нулиран!
dns-server-reset-failed = Неуспешно нулиране на DNS сървър!
winboat-install-failed = Неуспешно инсталиране на Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} активиран
tweak-psd-tooltip = Съхраняване на профилите на браузъра в RAM (по-бързо, намалява износването на диска)
tweak-oomd-tooltip = Проактивно прекратяване на процеси при недостиг на памет, за да се предотвратят забивания
tweak-bpftune-tooltip = Автоматична настройка на системната мрежа
tweak-bluetooth-tooltip = Активиране на поддръжка за Bluetooth устройства (мишки, аудио и др.)
tweak-ananicycpp-tooltip = Автоматично регулира приоритетите на процесите за по-добра отзивчивост на системата
tweak-cachyupdate-tooltip = Известие за обновления в системния трей

# Tweaks page (fixes)
update-system-title = АКТУАЛИЗИРАНЕ НА СИСТЕМАТА
remove-orphans-title = Премахване на изоставени пакети
rankmirrors-title = Класиране на mirror сървъри
dnsserver-title = Смяна на DNS сървър
install-gaming-title = Инсталиране на пакети за гейминг
install-winboat-title = Инсталиране на Winboat
install-vram-management-title = Инсталиране на управление на VRAM
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
button-readme-label = Информация
button-release-info-label = Информация за изданието
button-wiki-label = Wiki

# Main Page (sections)
section-docs = ДОКУМЕНТАЦИЯ
section-installer = ИНСТАЛАЦИЯ
section-support = ПОДДРЪЖКА
section-project = ПРОЕКТ

# Main Page (launch installer)
calamares-install-type = Тип инсталация Calamares

# Main Page (body)
offline-error = Не може да се стартира онлайн инсталацията! Няма интернет връзка.
unsupported-hw-warning = Опитвате се да инсталирате на хардуер, който не се поддържа от текущото ISO. Инсталацията няма да отговаря на условията за поддръжка.
desktop-on-handheld-error = Опитвате се да инсталирате настолната версия на преносимо устройство (handheld). Моля, използвайте версията Handheld за правилна поддръжка на този хардуер.
outdated-version-warning = Използвате по-стара версия на CachyOS ISO. Помислете да използвате най-новата версия за инсталации.
testing-iso-warning = Използвате тестов ISO образ. Тестовите ISO образи не се считат за стабилни и готови за употреба.
tweaksbrowser-label = Настройки и помощни програми
appbrowser-label = Инсталиране на приложения
troubleshooting-label = Отстраняване на проблеми
launch-start-label = Стартиране при включване
welcome-title = Добре дошли в CachyOS!
welcome-body =
    Благодарим ви, че се присъединихте към нашата общност!

    Надяваме се, че ще използвате CachyOS с удоволствие – точно толкова, колкото на нас ни доставя удоволствие да я създаваме. Връзките по-долу ще ви помогнат да започнете работа с новата си операционна система. Насладете се на работата с нея и не се колебайте да ни изпратите обратна връзка.
