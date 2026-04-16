# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Вітальний екран CachyOS

# Tweaks page
tweaks = Налаштування
fixes = Обслуговування
applications = Застосунки
removed-db-lock = Блокування бази даних Pacman знято.
lock-doesnt-exist = Блокування бази даних Pacman відсутнє!
orphans-not-found = Осиротілих пакетів не знайдено.
package-not-installed = Пакет '{$package_name}' не встановлено.
gaming-package-installed = Ігрові пакети вже встановлені!
winboat-package-installed = Пакети Winboat вже встановлені!
gpu-boosters-package-installed = Пакети прискорювачів GPU вже встановлені!

# Application Browser page
advanced-btn = розширені
reset-btn = скинути
update-system-app-btn = ОНОВИТИ СИСТЕМУ
application-column = Застосунок
description-column = Опис
install-remove-column = Встановити/Видалити
advanced-btn-tooltip = Показати розширений вибір пакетів
reset-btn-tooltip = Скинути поточний вибір…
update-system-app-btn-tooltip = Застосувати поточний вибір до системи

# DNS Connections page
dns-settings = Налаштування DNS
select-connection = Виберіть з’єднання:
select-dns-server = Виберіть DNS-сервер:
apply = Застосувати
reset = Скинути
enable-dot = Увімкнути DNS через TLS (DoT)
dot-tooltip = Шифрування DNS-запитів за допомогою TLS для кращої конфіденційності (потребує підтримки сервера)
enable-doh = Увімкнути DNS через HTTPS (DoH)
doh-tooltip = Шифрувати DNS-запити через HTTPS за допомогою локального проксі blocky (потрібна підтримка сервера, встановлюється blocky)
doh-blocky-install-failed = Не вдалося встановити blocky для підтримки DoH!
test-latency = Перевірити затримку
test-latency-tooltip = Виміряти мережеву затримку до обраного DNS-сервера
best-server = Обрати найкращий сервер за затримкою
best-server-tooltip = Перевірити базові DNS-сервери (без фільтрації) та обрати найшвидший
latency-result = {""}
server-info = {""}
latency-testing = тестування...
latency-timeout = тайм-аут
latency-no-result = жоден сервер не відповів
custom-dns = Власний
dhcp-automatic = DHCP (автоматично)
custom-dns-ipv4 = IPv4-адреси (через кому):
custom-dns-ipv6 = IPv6-адреси (через кому):
custom-dns-dot-hostname = Ім’я хоста DoT (необов’язково):
custom-dns-invalid = Введіть принаймні IPv4 або IPv6 адресу
custom-dns-invalid-hostname = Неправильне ім’я хоста DoT
custom-dns-doh-url = URL DoH (для DNS через HTTPS):
custom-dns-doh-url-required = Введіть коректний URL DoH, що починається з https://
dns-check-hint = Після застосування перевірте вашого DNS-провайдера на
dns-server-changed = DNS-сервер змінено.
dns-server-failed = Не вдалося змінити DNS-сервер!
dns-server-reset = Налаштування DNS скинуто!
dns-server-reset-failed = Не вдалося скинути налаштування DNS!
winboat-install-failed = Не вдалося встановити Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} увімкнено
tweak-psd-tooltip = Використовувати RAM для профілів браузера (швидше та менше зношує диск)
tweak-oomd-tooltip = Завчасно завершувати процеси за нестачі пам’яті, щоб уникнути зависань
tweak-bpftune-tooltip = Автоматично оптимізувати мережеві параметри системи
tweak-bluetooth-tooltip = Увімкнути підтримку Bluetooth-пристроїв (миші, навушники тощо)
tweak-ananicycpp-tooltip = Автоматично керувати пріоритетами процесів для кращої чутливості системи
tweak-cachyupdate-tooltip = Сповіщення про оновлення в панелі сповіщень

# Tweaks page (fixes)
remove-lock-title = Зняти блокування бази даних
reinstall-title = Перевстановити всі пакети
reset-keyrings-title = Скинути ключі підпису
update-system-title = Оновити систему
remove-orphans-title = Видалити осиротілі пакети
clear-pkgcache-title = Очистити кеш пакетів
rankmirrors-title = Оптимізувати дзеркала
dnsserver-title = Змінити DNS-сервер
show-kwinw-debug-title = Показати вікно налагодження KWin (Wayland)
install-gaming-title = Встановити ігрові пакети
install-winboat-title = Встановити Winboat
install-gpu-boosters-title = Встановити прискорювачі GPU
install-gpu-boosters-tooltip = Встановити dmemcg-booster і plasma-foreground-booster для GPU AMD або Intel

# Main Page (buttons)
button-about-tooltip = Про програму
button-web-resource-tooltip = Вебресурс
button-development-label = Розробка
button-software-label = Програмне забезпечення
button-donate-label = Підтримати
button-forum-label = Форум
button-installer-label = Запустити встановлення
button-involved-label = Долучитися
button-readme-label = Прочитати
button-release-info-label = Інформація про випуск
button-wiki-label = Вікі

# Main Page (sections)
section-docs = ДОКУМЕНТАЦІЯ
section-installer = ВСТАНОВЛЕННЯ
section-support = ПІДТРИМКА
section-project = ПРОЄКТ

# Main Page (launch installer)
recommended = рекомендовано
calamares-install-type = Тип встановлення Calamares

# Main Page (body)
offline-error = Неможливо запустити онлайн-встановлення — відсутнє інтернет-з’єднання
unsupported-hw-warning = Ви намагаєтеся встановити систему на обладнання, яке не підтримується поточним ISO. Така інсталяція не підлягатиме підтримці
desktop-on-handheld-error = Ви намагаєтеся встановити настільну версію на портативний пристрій. Використовуйте портативну версію для належної підтримки
outdated-version-warning = Ви використовуєте застарілу версію ISO CachyOS. Рекомендуємо скористатися найновішою
testing-iso-warning = Ви використовуєте тестове ISO. Тестові збірки не вважаються стабільними
tweaksbrowser-label = Застосунки / Налаштування
appbrowser-label = Встановити застосунки
launch-start-label = Запускати під час входу
welcome-title = Ласкаво просимо до CachyOS!
welcome-body =
    Дякуємо, що приєдналися до нашої спільноти!

    Ми, команда розробників CachyOS, сподіваємося, що вам буде так само приємно користуватися цією системою, як нам — створювати її. Посилання нижче допоможуть швидко розпочати роботу. Насолоджуйтесь і не соромтеся ділитися своїми відгуками.
