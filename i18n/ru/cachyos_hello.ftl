# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Приветственный экран CachyOS

# Tweaks page
tweaks = Настройки
fixes = Утилиты
applications = Приложения
removed-db-lock = Блокировка БД Pacman была снята!
lock-doesnt-exist = БД pacman не заблокирована!
orphans-not-found = Пакеты-сироты не найдены!
package-not-installed = Пакет '{$package_name}' не был установлен!
gaming-package-installed = Игровые пакеты уже установлены!
winboat-package-installed = Пакеты Winboat уже установлены!
vram-management-package-installed = Пакеты управления VRAM уже установлены!

# Application Browser page
advanced-btn = дополнительные
reset-btn = сброс
update-system-app-btn = ОБНОВИТЬ СИСТЕМУ
application-column = Приложение
description-column = Описание
install-remove-column = Установить/удалить
advanced-btn-tooltip = Переключить на расширенный выбор пакетов
reset-btn-tooltip = Сброс текущий выбор...
update-system-app-btn-tooltip = Применить текущий выбор к системе

# Dns Connections page
dns-settings = Настройки DNS
select-connection = Выберите подключение:
select-dns-server = Выберите DNS сервер:
apply = Применить
reset = Сбросить
enable-dot = Включить DNS через TLS (DoT)
dot-tooltip = Шифрование DNS-запросов с помощью TLS для повышения конфиденциальности (требуется поддержка сервера)
enable-doh = Включить DNS через HTTPS (DoH)
doh-tooltip = Шифрование DNS-запросов с помощью HTTPS через локальный прокси blocky (требуется поддержка сервера, устанавливает blocky)
doh-blocky-install-failed = Не удалось установить blocky для поддержки DoH!
test-latency = Тест задержки выбранного сервера
test-latency-tooltip = Измерить сетевую задержку до выбранного DNS-сервера
best-server = Выбрать лучший сервер по задержке
best-server-tooltip = Протестировать базовые DNS-серверы (без фильтрующих вариантов) и выбрать самый быстрый
latency-result = {""}
server-info = {""}
latency-testing = тестирование...
latency-timeout = тайм-аут
latency-no-result = ни один сервер не ответил
custom-dns = Пользовательский
dhcp-automatic = DHCP (автоматически)
custom-dns-ipv4 = Адреса IPv4 (через запятую):
custom-dns-ipv6 = Адреса IPv6 (через запятую):
custom-dns-dot-hostname = Имя хоста DoT (необязательно):
custom-dns-invalid = Пожалуйста, введите хотя бы один адрес IPv4 или IPv6
custom-dns-invalid-hostname = Неверное имя хоста DoT
custom-dns-doh-url = URL DoH (для DNS через HTTPS):
custom-dns-doh-url-required = Пожалуйста, введите корректный URL DoH, начинающийся с https://
dns-check-hint = После применения проверьте вашего DNS-провайдера на
dns-server-changed = DNS-сервер был успешно изменен!
dns-server-failed = Не удалось настроить DNS-сервер!
dns-server-reset = DNS-сервер был сброшен!
dns-server-reset-failed = Не удалось сбросить DNS-сервер!
winboat-install-failed = Не удалось установить Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} включен
tweak-psd-tooltip = Использовать ОЗУ для профилей браузера (быстрее, меньше износ диска)
tweak-oomd-tooltip = Принудительно завершать процессы при нехватке памяти для предотвращения зависаний
tweak-bpftune-tooltip = Автоматически настраивать сетевые параметры системы
tweak-bluetooth-tooltip = Включить поддержку беспроводных Bluetooth-устройств (мыши, аудио и т.д.)
tweak-ananicycpp-tooltip = Автоматически регулировать приоритеты процессов для улучшения отзывчивости системы
tweak-cachyupdate-tooltip = Уведомитель обновлений в трее


# Tweaks page (fixes)
remove-lock-title = Удалить файл блокировки БД pacman
reinstall-title = Переустановить все пакеты
reset-keyrings-title = Сбросить ключи
update-system-title = Обновить систему
remove-orphans-title = Удалить пакеты-сироты
clear-pkgcache-title = Очистить кэш пакетов
rankmirrors-title = Ранжировать зеркала
dnsserver-title = Сменить DNS-сервер
show-kwinw-debug-title = Показать окно отладки KWin для Wayland
install-gaming-title = Установить пакеты для игр
install-winboat-title = Установить Winboat
install-vram-management-title = Установить пакеты управления VRAM
install-vram-management-tooltip = Приоритизировать VRAM для активного приложения, чтобы драйвер GPU не переносил буферы в системную ОЗУ (GTT).


# Main Page (buttons)
button-about-tooltip = О программе
button-web-resource-tooltip = Веб-ресурс
button-development-label = Разработка
button-software-label = ПО
button-donate-label = Пожертвовать
button-forum-label = Форум
button-installer-label = Запустить установщик
button-involved-label = Принять участие
button-readme-label = Прочитай меня
button-release-info-label = Сведения о выпуске
button-wiki-label = Вики

# Main Page (sections)
section-docs = ДОКУМЕНТАЦИЯ
section-installer = УСТАНОВКА
section-support = ПОДДЕРЖКА
section-project = ПРОЕКТ

# Main Page (launch installer)
recommended = рекомендовано
calamares-install-type = Calamares тип установки

# Main Page (body)
offline-error = Не удается запустить онлайн-установку! Нет подключения к Интернету
unsupported-hw-warning = Вы пытаетесь установить систему на оборудовании, не поддерживаемом текущим ISO. Ваша установка не будет иметь поддержки
desktop-on-handheld-error = Вы пытаетесь установить редакцию Desktop на портативном устройстве. Пожалуйста, используйте редакцию Handheld для надлежащей поддержки на этом оборудовании
outdated-version-warning = Вы используете устаревшую версию CachyOS ISO, пожалуйста, рассмотрите возможность использования последней версии для установки
testing-iso-warning = Вы используете тестовую версию ISO, тестовые версии ISO не считаются стабильными и готовыми к использованию
tweaksbrowser-label = Приложения/Настройки
appbrowser-label = Установить ПO
launch-start-label = Автозапуск
welcome-title = Добро пожаловать в CachyOS!
welcome-body =
    Благодарим Вас за то, что Вы присоединились к нашему сообществу!

    Мы, разработчики CachyOS, надеемся, что пользуясь этой системой, Вы будете испытывать такое же удовольствие, какое испытывали мы, создавая ее. Представленные ниже ссылки помогут Вам начать работу. Наслаждайтесь функционалом CachyOS и оставляйте свои отзывы.
