# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = صفحه خوش‌آمدگویی CachyOS

# Tweaks page
tweaks = تنظیمات
fixes = ابزارها
applications = برنامه‌ها
removed-db-lock = قفل پایگاه‌داده Pacman حذف شد!
lock-doesnt-exist = قفل پایگاه‌داده Pacman وجود ندارد!
orphans-not-found = هیچ بسته یتیمی پیدا نشد!
package-not-installed = بسته «{$package_name}» نصب نشده است!
gaming-package-installed = بسته‌های گیمینگ از قبل نصب شده‌اند!
winboat-package-installed = بسته‌های Winboat از قبل نصب شده‌اند!
vram-management-package-installed = بسته‌های مدیریت VRAM از قبل نصب شده‌اند!

# Application Browser page
advanced-btn = پیشرفته
reset-btn = بازنشانی
update-system-app-btn = به‌روزرسانی سیستم
application-column = برنامه
description-column = توضیحات
install-remove-column = نصب/حذف
advanced-btn-tooltip = نمایش یا پنهان‌کردن گزینش گسترده‌تری از بسته‌ها
reset-btn-tooltip = بازنشانی انتخاب‌های فعلی شما...
update-system-app-btn-tooltip = اعمال انتخاب‌های فعلی شما روی سیستم

# Troubleshooting page
troubleshooting = عیب‌یابی

# Dns Connections page
dns-settings = تنظیمات DNS
select-connection = انتخاب اتصال:
select-dns-server = انتخاب سرور DNS:
apply = اعمال
reset = بازنشانی
enable-encrypted-dns = فعال‌سازی DNS رمزنگاری‌شده روی {$protocol} ({$abbr})
dot-tooltip = رمزنگاری درخواست‌های DNS با استفاده از TLS برای حریم خصوصی بهتر (نیازمند پشتیبانی سرور)
blocky-dns-tooltip = رمزنگاری درخواست‌های DNS با استفاده از {$protocol} از طریق پراکسی محلی blocky (نیازمند پشتیبانی سرور، blocky را نصب می‌کند)
blocky-install-failed = نصب blocky برای پشتیبانی از {$mode} ناموفق بود!
test-latency = آزمایش تأخیر سرور انتخاب‌شده
test-latency-tooltip = اندازه‌گیری تأخیر شبکه نسبت به سرور DNS انتخاب‌شده
best-server = انتخاب بهترین سرور بر اساس تأخیر
best-server-tooltip = آزمایش سرورهای DNS پایه (به‌جز نسخه‌های فیلترکننده) و انتخاب سریع‌ترین آن‌ها
latency-result = {""}
server-info = {""}
latency-testing = در حال آزمایش...
latency-timeout = اتمام زمان
latency-no-result = هیچ سروری پاسخ نداد
custom-dns = سفارشی
dhcp-automatic = DHCP (خودکار)
custom-dns-ip = آدرس‌های {$version} (با کاما جدا شوند):
custom-dns-dot-hostname = نام میزبان DoT (اختیاری):
custom-dns-invalid = لطفاً حداقل یک آدرس IPv4 یا IPv6 وارد کنید
custom-dns-invalid-hostname = نام میزبان DoT نامعتبر است
custom-dns-doh-url = آدرس DoH (برای DNS از طریق HTTPS):
custom-dns-doh-url-required = لطفاً یک آدرس DoH معتبر که با https:// شروع می‌شود وارد کنید
custom-dns-doq-endpoint = نقطه پایانی DoQ (برای DNS از طریق QUIC):
custom-dns-doq-endpoint-required = لطفاً یک نقطه پایانی DoQ معتبر که با quic: یا quic:// شروع می‌شود وارد کنید
dns-check-hint = پس از اعمال، ارائه‌دهنده DNS خود را در {$dnscheck_url} بررسی کنید
dns-server-changed = سرور DNS با موفقیت تغییر یافت!
dns-server-failed = تنظیم سرور DNS ناموفق بود!
dns-server-reset = سرور DNS بازنشانی شد!
dns-server-reset-failed = بازنشانی سرور DNS ناموفق بود!
winboat-install-failed = نصب Winboat ناموفق بود!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} فعال شد
tweak-psd-tooltip = استفاده از RAM برای پروفایل‌های مرورگر (سریع‌تر، فشار کمتر روی دیسک)
tweak-oomd-tooltip = پایان‌دادن فعالانه به پردازه‌ها هنگام کمبود حافظه برای پیشگیری از هنگ‌کردن سیستم
tweak-bpftune-tooltip = تنظیم خودکار شبکه سیستم
tweak-bluetooth-tooltip = فعال‌سازی پشتیبانی از دستگاه‌های بی‌سیم بلوتوث (موس، صدا و غیره)
tweak-ananicycpp-tooltip = تنظیم خودکار اولویت پردازه‌ها برای پاسخ‌گویی بهتر سیستم
tweak-cachyupdate-tooltip = اعلان‌دهنده به‌روزرسانی در سینی سیستم

# Tweaks page (fixes)
remove-lock-title = حذف قفل پایگاه‌داده
reinstall-title = نصب مجدد همه بسته‌ها
reset-keyrings-title = بازنشانی کلیدهای امنیتی (keyrings)
update-system-title = به‌روزرسانی سیستم
remove-orphans-title = حذف بسته‌های یتیم
clear-pkgcache-title = پاک‌سازی حافظه نهان بسته‌ها
rankmirrors-title = رتبه‌بندی آینه‌ها (mirrors)
dnsserver-title = تغییر سرور DNS
show-kwinw-debug-title = نمایش پنجره اشکال‌زدایی kwin (Wayland)
install-gaming-title = نصب بسته‌های گیمینگ
install-winboat-title = نصب Winboat
install-vram-management-title = نصب مدیریت VRAM
install-vram-management-tooltip = اولویت‌دهی VRAM برای برنامه پیش‌زمینه تا درایور گرافیک از سرریز‌شدن بافرها به RAM سیستم (GTT) جلوگیری کند.

# Main Page (buttons)
button-about-tooltip = درباره
button-web-resource-tooltip = منبع وب
button-development-label = توسعه
button-software-label = نرم‌افزار
button-donate-label = اهدای مالی
button-forum-label = انجمن
button-installer-label = اجرای نصب‌کننده
button-involved-label = مشارکت کنید
button-readme-label = راهنما
button-release-info-label = اطلاعات نسخه
button-wiki-label = ویکی

# Main Page (sections)
section-docs = مستندات
section-installer = نصب
section-support = پشتیبانی
section-project = پروژه

# Main Page (launch installer)
recommended = پیشنهادی
calamares-install-type = نوع نصب Calamares

# Main Page (body)
offline-error = راه‌اندازی نصب آنلاین ممکن نیست! اتصال اینترنت برقرار نیست
unsupported-hw-warning = شما در حال نصب روی سخت‌افزاری هستید که توسط این ISO پشتیبانی نمی‌شود؛ نصب شما واجد شرایط پشتیبانی نخواهد بود
desktop-on-handheld-error = شما در حال نصب نسخه دسکتاپ روی یک دستگاه دستی (handheld) هستید. لطفاً برای پشتیبانی مناسب از این سخت‌افزار، از نسخه Handheld استفاده کنید
outdated-version-warning = شما از نسخه قدیمی‌تری از ISO سیستم CachyOS استفاده می‌کنید؛ لطفاً برای نصب از آخرین نسخه استفاده کنید
testing-iso-warning = شما از یک ISO آزمایشی استفاده می‌کنید؛ ISOهای آزمایشی پایدار و آماده استفاده در نظر گرفته نمی‌شوند
tweaksbrowser-label = برنامه‌ها/تنظیمات
appbrowser-label = نصب برنامه‌ها
troubleshooting-label = عیب‌یابی
launch-start-label = اجرا هنگام شروع سیستم
welcome-title = به CachyOS خوش آمدید!
welcome-body =
    از پیوستن شما به انجمن ما سپاسگزاریم!

    ما، توسعه‌دهندگان CachyOS، امیدواریم به همان اندازه که ما از ساختن CachyOS لذت می‌بریم، شما هم از استفاده از آن لذت ببرید. پیوندهای زیر به شما کمک می‌کنند تا با سیستم‌عامل جدید خود شروع کنید. پس از این تجربه لذت ببرید و در ارسال بازخورد خود به ما تردید نکنید.
