# About dialog
about-dialog-title = CachyOS 欢迎
about-dialog-comments = CachyOS 的欢迎界面

# Tweaks page
tweaks = 调整
fixes = 实用工具
applications = 应用程序
removed-db-lock = Pacman 数据库锁已被移除！
lock-doesnt-exist = Pacman 数据库锁不存在！
orphans-not-found = 没有发现孤立软件包！
package-not-installed = 软件包 '{$package_name}' 没有被安装！
gaming-package-installed = 游戏软件包已经安装！
winboat-package-installed = Winboat 已经安装！
vram-management-package-installed = VRAM 管理软件包已经安装！

# Application Browser page
advanced-btn = 高级选项
reset-btn = 重置
update-system-app-btn = 更新系统
application-column = 应用程序
description-column = 描述
install-remove-column = 安装/移除
advanced-btn-tooltip = 显示软件包的附加选项
reset-btn-tooltip = 重置您的选项...
update-system-app-btn-tooltip = 向系统应用您的选项

# Dns Connections page
dns-settings = DNS 设置
select-connection = 选择网络:
select-dns-server = 选择 DNS 服务器:
apply = 应用
reset = 重置
enable-dot = 启用 DNS over TLS (DoT)
dot-tooltip = 使用 TLS 加密 DNS 查询来提高隐私性（需要服务器支持）
enable-doh = 启用 DNS over HTTPS (DoH)
doh-tooltip = 使用 Blocky 通过 HTTPS 加密 DNS 查询 (需要服务器支持，并安装 blocky)
doh-blocky-install-failed = 无法为 DoH 支持安装 blocky！
test-latency = 测试所选服务器的延迟
test-latency-tooltip = 为选择的 DNS 服务器测量网络延迟
best-server = 通过延迟来选择最佳的服务器
best-server-tooltip = 测试基准 DNS 服务器（排除带过滤功能的变体），并选择最快的一个
latency-result = {""}
server-info = {""}
latency-testing = 测试中...
latency-timeout = 超时
latency-no-result = 所有服务器均未回应
custom-dns = 自定义
dhcp-automatic = DHCP（自动）
custom-dns-ipv4 = IPv4 地址 (以逗号分隔):
custom-dns-ipv6 = IPv6 地址 (以逗号分隔):
custom-dns-dot-hostname = DoT 服务器主机名（可选）:
custom-dns-invalid = 请输入至少一个 IPv4 或 IPv6 地址
custom-dns-invalid-hostname = 无效的 DoT 主机名
custom-dns-doh-url = DoH URL（对于 DNS over HTTPS):
custom-dns-doh-url-required = 请输入一个以 https:// 开头的，有效的 DoH URL 
dns-check-hint = 应用后，在这里验证您的 DNS 提供商
dns-server-changed = DNS 服务器已成功修改！
dns-server-failed = 无法设置 DNS 服务器！
dns-server-reset = DNS 服务器已经重置！
dns-server-reset-failed = 无法重置 DNS 服务器！
winboat-install-failed = 无法安装 Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = 启用 {$tweak}
tweak-psd-tooltip = 使用 RAM 来存放浏览器配置文件（更快，更少的磁盘磨损)
tweak-oomd-tooltip = 在内存不足时主动终止进程，以防止系统卡死/冻结。
tweak-bpftune-tooltip = 自动调优系统网络
tweak-bluetooth-tooltip = 启用对于蓝牙设备的支持（鼠标，音频设备及更多)
tweak-ananicycpp-tooltip = 自动调整进程优先级以提升系统响应能力
tweak-cachyupdate-tooltip = 托盘上的更新提示

# Tweaks page (fixes)
remove-lock-title = 移除数据库锁
reinstall-title = 重新安装所有软件包
reset-keyrings-title = 重置密钥环
update-system-title = 系统更新
remove-orphans-title = 移除孤立软件包
clear-pkgcache-title = 清除软件包缓存
rankmirrors-title = 排序镜像
dnsserver-title = 更改 DNS 服务器
show-kwinw-debug-title = 显示 kwin(Wayland) 调试控制台
install-gaming-title = 安装游戏软件包
install-winboat-title = 安装 Winboat
install-vram-management-title = 安装显存（VRAM）管理功能
install-vram-management-tooltip = 将显存优先分配给前台应用程序，使 GPU 驱动避免将缓冲区外溢到到系统内存（GTT）中。

# Main Page (buttons)
button-about-tooltip = 关于
button-web-resource-tooltip = 网络资源
button-development-label = 开发
button-software-label = 软件
button-donate-label = 捐赠
button-forum-label = 论坛
button-installer-label = 启动安装程序
button-involved-label = 参与进来
button-readme-label = README
button-release-info-label = 发布信息
button-wiki-label = Wiki

# Main Page (sections)
section-docs = 文档
section-installer = 安装
section-support = 支持
section-project = 项目

# Main Page (launch installer)
recommended = 建议
calamares-install-type = Calamares 安装类型

# Main Page (body)
offline-error = 无法开始在线安装！无网络连接
unsupported-hw-warning = 您正尝试安装在当前 ISO 不支持的硬件上，您的安装将不会被合法地支持
desktop-on-handheld-error = 您正尝试在手持设备上安装桌面版。请使用手持版以获得与此硬件的良好兼容性。
outdated-version-warning = 您正使用 CachyOS 的一个旧版 ISO，请考虑使用最新版来安装
testing-iso-warning = 您正使用测试版的 ISO，测试版 ISO 尚未稳定，不建议日常使用
tweaksbrowser-label = 应用/调整
appbrowser-label = 安装应用
launch-start-label = 在启动时打开
welcome-title = 欢迎使用 CachyOS！
welcome-body =
    欢迎您加入我们的社区！

    我们是 CachyOS 的开发者，希望您能像我们开发一样愉快地使用 CachyOS！下方的链接将帮助您开始探索您的新操作系统，请尽管享受您的体验，如果您有任何的反馈或意见，不用犹豫！发送给我们！
