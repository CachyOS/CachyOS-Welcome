# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = CachyOS-ის მისალმების ეკრანი

# Tweaks page
tweaks = ცვლილებები
fixes = ხელსაწყოები
applications = აპლიკაციები
removed-db-lock = Pacman-ის მონაცემთა ბაზის ბლოკი მოხსნილია!
lock-doesnt-exist = Pacman-ის მონაცემთა ბაზის ბლოკი არ არსებობს!
orphans-not-found = ობოლი ფექიჯები აღმოჩენილი არაა!
package-not-installed = ფექიჯი '{$package_name}' დაყენებული არაა!
gaming-package-installed = თამაშის მხარდაჭერის ფექიჯები უკვე დაყენებულია!
winboat-package-installed = Winboat-ის ფექიჯები უკვე დაყენებულია!
vram-management-package-installed = VRAM მენეჯმენტის ფექიჯები უკვე დაყენებულია!

# Application Browser page
advanced-btn = გაფართოებული
reset-btn = განულება
update-system-app-btn = სისტემის განახლება
application-column = აპლიკაცია
description-column = აღწერა
install-remove-column = დაყენება/წაშლა
advanced-btn-tooltip = ფექიჯების გაფართოებული არჩევანის გადართვა
reset-btn-tooltip = მიმდინარე არჩევანის განულება...
update-system-app-btn-tooltip = გამოიყენეთ თქვენი მიმდინარე არჩევანი სისტემაზე

# Troubleshooting page
troubleshooting = პრობლემების მოგვარება

# Dns Connections page
dns-settings = DNS-ის მორგება
select-connection = აირჩიეთ კავშირი:
select-dns-server = აირჩიეთ DNS სერვერი:
apply = მიღება/გამოყენება
reset = განულება
enable-encrypted-dns = DNS {$protocol} ({$abbr}) მეშვეობით ჩართვა
dns-type-label = DNS ტიპი:
dns-type-tooltip = აირჩიეთ როგორ გადაადგილდება DNS მოთხოვნები. დაშიფრული ტიპები (DoT/DoH/DoQ) მოითხოვენ სერვერის მხარდაჭერას; DoH and DoQ იყენებს blocky ადგილობრივ პროქსს.
dns-type-plain = უბრალო (დაუშიფრავი)
dns-type-dot = DNS TLS-ის მეშვეობით (DoT)
dns-type-doh = DNS HTTPS-ის მეშვეობით (DoH)
dns-type-doq = DNS over QUIC-ის მეშვეობით (DoQ)
dot-tooltip = DNS მოთხოვნების დაშიფვრა TLS-ის გამოყენებით გაუმჯობესებული კონფიდენციალობისთვის (საჭიროებს სერვერის მხარდაჭერას)
blocky-dns-tooltip = DNS მოთხოვნების დაშიფვრა {$protocol}-ის გამოყენებით blocky ლოკალური პროქსის მეშვეობით (საჭიროებს სერვერის მხარდაჭერას, აყენებს blocky-ს)
blocky-install-failed = blocky-ის დაყენება ვერ მოხერხდა {$mode}-ის მხარდაჭერისთვის!
test-latency = არჩეული სერვერის დაყოვნების შემოწმება
test-latency-tooltip = ქსელის დაყოვნების გაზომვა არჩეულ DNS სერვერამდე
best-server = საუკეთესო სერვერის შერჩევა დაყოვნების მიხედვით
best-server-tooltip = ძირითადი DNS სერვერების შემოწმება(გაფილტრული ვარიანტების გამოტოვებით) და ყველაზე სწრაფის არჩევა
latency-result = {""}
server-info = {""}
latency-testing = მიმდინარეობს შემოწმება...
latency-timeout = მოლოდინის დრო ამოიწურა
latency-no-result = სერვერის პასუხის გარეშე
custom-dns = მორგებული
dhcp-automatic = DHCP (აუტომატური)
custom-dns-ip = {$version} მისამართები (მძიმე-გამოტოვებით):
custom-dns-dot-hostname = DoT hostname (არააუცილებელი):
custom-dns-invalid = გთხოვთ შეიყვანოთ IPv4 ან IPv6 მისამართი
custom-dns-invalid-hostname = არავალიდური DoT hostname
custom-dns-doh-url = DoH URL (DNS HTTPS-ის მეშვეობით):
custom-dns-doh-url-required = გთხოვთ შეიყვანოთ ვალიდური DoH URL რომელიც იწყება https://
custom-dns-doq-endpoint = DoQ endpoint (DNS QUIC-ის მეშვეობით):
custom-dns-doq-endpoint-required = გთხოვთ შეიყვანოთ ვალიდური DoQ endpoint რომელიც იწყება quic: or quic://
dns-check-hint = გამოყენების შემდეგ, გადაამოწმეთ თქვენი DNS პროვაიდერი აქ: {$dnscheck_url}
dns-server-changed = DNS სერვერი წარმატებით შეიცვალა!
dns-server-pending = DNS სერვერი შეინახა. ხელახლა მიუერთდით ქსელს (ან ჩართე/გამორთეთ) ცვლილებების ასახვისთვის.
dns-server-failed = DNS სერვერის დაყენება ვერ მოხერხდა!
dns-server-reset = DNS სერვერის პარამეტრები განულებულია!
dns-server-reset-failed = DNS სერვერის განულება ვერ მოხერხდა!
winboat-install-failed = Winboat-ის დაყენება ვერ მოხერხდა!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} ჩართულია
tweak-psd-tooltip = ბრაუზერის პროფილებისთვის ოპერატიული მეხსიერების (RAM) გამოყენება (უფრო სწრაფია, ნაკლებად ცვეთს დისკს)
tweak-oomd-tooltip = პროცესების პრევენციული გათიშვა მეხსიერების ნაკლებობისას სისტემის გაჭედვის თავიდან ასაცილებლად
tweak-bpftune-tooltip = სისტემის ქსელის ავტომატური ოპტიმიზაცია
tweak-bluetooth-tooltip = ბლუთუზის უსადენო მოწყობილობების მხარდაჭერის ჩართვა (თაგუნა, აუდიო და ა.შ.)
tweak-ananicycpp-tooltip = პროცესების პრიორიტეტების ავტომატური რეგულირება სისტემის უკეთესი რეაგირებისთვის
tweak-cachyupdate-tooltip = განახლებების შესახებ შეტყობინება სისტემურ პანელში

# Tweaks page (fixes)
remove-lock-title = DB დაბლოკვის მოხსნა
reinstall-title = ყველა ფექიჯის თავიდან დაყენება
reset-keyrings-title = Keyrings-ის განულება
update-system-title = სისტემის განახლება
remove-orphans-title = ობოლი ფექიჯების წაშლა
clear-pkgcache-title = ფექიჯების ქეშის გასუფთავება
rankmirrors-title = სარკეების რანკირება
dnsserver-title = DNS სერვერის შეცვლა
show-kwinw-debug-title = kwin(Wayland) debug ფანჯრის ჩვენება
install-gaming-title = თამაშის ფექიჯების დაყენება
install-winboat-title = Winboat-ის დაყენება
install-vram-management-title = VRAM მენეჯმენტის დაყენება
install-vram-management-tooltip = წინა პლანზე განთავსებული აპლიკაციისთვის VRAM-ს მიანიჭეთ პრიორიტეტი, რათა GPU დრაივერმა თავიდან აიცილოს ბუფერები სისტემის ოპერატიულ მეხსიერებაში (GTT).

# Main Page (buttons)
button-about-tooltip = შესახებ
button-web-resource-tooltip = ვებ-რესურსი
button-development-label = დეველოპმენტი
button-software-label = პროგრამები
button-donate-label = დონაცია
button-forum-label = ფორუმი
button-installer-label = Installer-ის გაშვება
button-involved-label = შემოგვიერთდით
button-readme-label = წაიკითხეთ
button-release-info-label = ინფორმაცია გამოშვებაზე
button-wiki-label = ვიკი

# Main Page (sections)
section-docs = დოკუმენტაცია
section-installer = დაყენება
section-support = მხარდაჭერა
section-project = პროექტი

# Main Page (launch installer)
recommended = რეკომენდირებული
calamares-install-type = Calamares-ის დაყენების ტიპი

# Main Page (body)
offline-error = ინტერნეტიდან დაყენების დაწყება შეუძლებელია! ინტერნეტთან კავშირი არ არსებობს
unsupported-hw-warning = თქვენ ცდილობთ დაყენებას მოწყობილობაზე, რომელიც ამ ISO-ის მიერ მხარდაჭერილი არაა. თუ მაინც დააყენებთ, მხარდაჭერის მიღების უფლება არ გექნებათ
desktop-on-handheld-error = თქვენ ცდილობთ, კომპიუტერის ვერსია პორტატულ მოწყობილობაზე დააყენოთ. აპარატურის სათანადო მხარდაჭერისთვის გამოიყენეთ სისტემის პორტატული ვერსია
outdated-version-warning = თქვენ იყენებთ CachyOS-ის ISO-ის ძველ ვერსიას. სჯობს, ახალი ვერსია გადმოწეროთ
testing-iso-warning = თქვენ იყენებთ სატესტო ISO ფაილს. სატესტო ISO ფაილები სტაბილურად არ ითვლება და ყოველდღიური მოხმარებისთვის რეკომენდირებული არაა
tweaksbrowser-label = აპები/ცვლილებები
appbrowser-label = აპების დაყენება
troubleshooting-label = პრობლემების მოგვარება
launch-start-label = გაშვება ჩართვისას
welcome-title = კეთილი იყოს თქვენი მობრძანება CachyOS-ში!
welcome-body =
    მადლობა, რომ შემოუერთდით ჩვენს საზოგადოებას!

    ჩვენ, CachyOS-ის დეველოპერები, ვიმედოვნებთ, რომ თქვენ ისე ისიამოვნებთ CachyOS-ით, როგორც ჩვენ მისი აგებისას. ბმულები ქვემოთ დაგეხმარებათ, თქვენს ახალ ოპერაციულ სისტემასთან მუშაობა დაიწყოთ. ასე რომ, ისიამოვნეთ და ნუ შეიკავებთ თავს, თქვენი აზრი გაგვიზიაროთ.