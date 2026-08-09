# About dialog
about-dialog-title = CachyOS 歡迎
about-dialog-comments = CachyOS 的歡迎介面

# Tweaks page
tweaks = 調整
fixes = 實用工具
applications = 應用程式
removed-db-lock = Pacman 資料庫鎖已被移除！
lock-doesnt-exist = Pacman 資料庫鎖不存在！
orphans-not-found = 未發現孤立套件！
package-not-installed = 套件 '{$package_name}' 尚未安裝！
gaming-package-installed = 遊戲套件已安裝！
winboat-package-installed = Winboat 已安裝！
vram-management-package-installed = VRAM 管理套件已安裝！


# Dns Connections page
dns-settings = DNS 設定
select-connection = 選擇網路：
select-dns-server = 選擇 DNS 伺服器：
apply = 套用
reset = 重設
enable-encrypted-dns = 啟用 DNS over {$protocol} ({$abbr})
blocky-install-failed = 無法為 {$mode} 支援安裝 blocky！
test-latency = 測試所選伺服器的延遲
test-latency-tooltip = 測量所選 DNS 伺服器的網路延遲
best-server = 依延遲選擇最佳伺服器
best-server-tooltip = 測試基準 DNS 伺服器（排除具備過濾功能的變體），並選擇最快的一個
latency-result = {""}
server-info = {""}
latency-testing = 測試中…
latency-timeout = 逾時
latency-no-result = 所有伺服器均未回應
custom-dns = 自訂
dhcp-automatic = DHCP（自動）
custom-dns-ip = {$version} 位址（以逗號分隔）：
custom-dns-dot-hostname = DoT 伺服器主機名稱（選填）：
custom-dns-invalid = 請輸入至少一個 IPv4 或 IPv6 位址
custom-dns-invalid-hostname = 無效的 DoT 主機名稱
custom-dns-doh-url = DoH URL（用於 DNS over HTTPS）：
custom-dns-doh-url-required = 請輸入以 https:// 開頭的有效 DoH URL
dns-check-hint = 套用後，在此驗證您的 DNS 提供商 {$dnscheck_url}
dns-server-changed = DNS 伺服器已成功修改！
dns-server-failed = 無法設定 DNS 伺服器！
dns-server-reset = DNS 伺服器已重設！
dns-server-reset-failed = 無法重設 DNS 伺服器！
winboat-install-failed = 無法安裝 Winboat！

# Tweaks page (tweaks)
tweak-enabled-title = 啟用 {$tweak}
tweak-psd-tooltip = 使用 RAM 存放瀏覽器設定檔（更快速、更少磁碟磨損）
tweak-oomd-tooltip = 在記憶體不足時主動終止程序，以防止系統凍結／卡死。
tweak-bpftune-tooltip = 自動調校系統網路
tweak-bluetooth-tooltip = 啟用藍牙裝置支援（滑鼠、音訊裝置等）
tweak-ananicycpp-tooltip = 自動調整程序優先權以提升系統回應速度
tweak-cachyupdate-tooltip = 系統列上的更新提示

# Tweaks page (fixes)
remove-lock-title = 移除資料庫鎖
reinstall-title = 重新安裝所有套件
reset-keyrings-title = 重設金鑰圈
update-system-title = 系統更新
remove-orphans-title = 移除孤立套件
clear-pkgcache-title = 清除套件快取
rankmirrors-title = 排序鏡像站
dnsserver-title = 變更 DNS 伺服器
show-kwinw-debug-title = 顯示 kwin (Wayland) 除錯主控台
install-gaming-title = 安裝遊戲套件
install-winboat-title = 安裝 Winboat
install-vram-management-title = 安裝顯示記憶體（VRAM）管理功能
install-vram-management-tooltip = 將顯示記憶體優先分配給前台應用程式，避免 GPU 驅動程式將緩衝區外溢至系統記憶體（GTT）中。

# Main Page (buttons)
button-about-tooltip = 關於
button-web-resource-tooltip = 網路資源
button-development-label = 開發
button-software-label = 軟體
button-donate-label = 贊助
button-forum-label = 論壇
button-installer-label = 啟動安裝程式
button-involved-label = 參與我們
button-readme-label = README
button-release-info-label = 發行資訊
button-wiki-label = Wiki

# Main Page (sections)
section-docs = 文件
section-installer = 安裝
section-support = 支援
section-project = 專案

# Main Page (launch installer)
calamares-install-type = Calamares 安裝類型

# Main Page (body)
offline-error = 無法開始線上安裝！無網路連線
unsupported-hw-warning = 您正嘗試安裝在目前 ISO 不支援的硬體上，您的安裝將不會受到正式支援
desktop-on-handheld-error = 您正嘗試在掌上型裝置上安裝桌面版。請使用掌機版以獲得良好的硬體相容性。
outdated-version-warning = 您正使用舊版的 CachyOS ISO，請考慮使用最新版本進行安裝
testing-iso-warning = 您正使用測試版的 ISO，測試版 ISO 尚未穩定，不建議日常使用
tweaksbrowser-label = 應用程式／調整
appbrowser-label = 安裝應用程式
launch-start-label = 開機時開啟
welcome-title = 歡迎使用 CachyOS！
welcome-body =
    歡迎您加入我們的社群！

    我們是 CachyOS 的開發者，希望您能像我們開發時一樣愉快地使用 CachyOS！下方的連結將幫助您開始探索您的新作業系統，請盡情享受您的體驗，如果您有任何回饋或意見，請隨時發送給我們！