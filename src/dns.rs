use std::sync::Arc;

use clap::{Subcommand, ValueEnum};
use phf::{phf_map, phf_ordered_map};

/// DNS server entry: (IPv4 addresses, IPv6 addresses, optional `DoT` hostname)
pub type DnsEntry = (&'static str, &'static str, Option<&'static str>);

/// Extra metadata for each DNS server (region, homepage URL, filtering variant).
pub struct DnsServerInfo {
    pub region: &'static str,
    pub homepage: &'static str,
    pub is_filtering: bool,
}

/// `DoH` URL for servers that support DNS over HTTPS.
pub static G_DNS_DOH_URLS: phf::Map<&'static str, &'static str> = phf_map! {
    "AdGuard" => "https://dns.adguard-dns.com/dns-query",
    "AdGuard Family Protection" => "https://family.adguard-dns.com/dns-query",
    "Cisco Umbrella(OpenDNS)" => "https://doh.opendns.com/dns-query",
    "Cloudflare" => "https://cloudflare-dns.com/dns-query",
    "Cloudflare Malware and adult content blocking" => "https://family.cloudflare-dns.com/dns-query",
    "Cloudflare Malware blocking" => "https://security.cloudflare-dns.com/dns-query",
    "FFMUC DNS / Freie Netze Muenchen e.V." => "https://doh.ffmuc.net/dns-query",
    "Google" => "https://dns.google/dns-query",
    "Quad9" => "https://dns.quad9.net/dns-query",
    "Yandex" => "https://common.dot.dns.yandex.net/dns-query",
    "Yandex Malware and adult content blocking" => "https://family.dot.dns.yandex.net/dns-query",
    "Yandex Malware blocking" => "https://safe.dot.dns.yandex.net/dns-query",
    "阿里云公共DNS (AliDNS)" => "https://dns.alidns.com/dns-query",
    "腾讯云 DNSPod (Tencent)" => "https://doh.pub/dns-query",
};

/// `DoQ` endpoint for servers that support DNS over QUIC (RFC 9250).
pub static G_DNS_DOQ_ENDPOINTS: phf::Map<&'static str, &'static str> = phf_map! {
    "AdGuard" => "quic:dns.adguard-dns.com",
    "AdGuard Family Protection" => "quic:family.adguard-dns.com",
    "FFMUC DNS / Freie Netze Muenchen e.V." => "quic:doq.ffmuc.net",
    "Quad9" => "quic:dns.quad9.net",
};

pub static G_DNS_SERVERS: phf::OrderedMap<&'static str, DnsEntry> = phf_ordered_map! {
    "AdGuard" => ("94.140.14.14,94.140.15.15", "2a10:50c0::ad1:ff,2a10:50c0::ad2:ff", Some("dns.adguard-dns.com")),
    "AdGuard Family Protection" => ("94.140.14.15,94.140.15.16", "2a10:50c0::bad1:ff,2a10:50c0::bad2:ff", Some("family.adguard-dns.com")),
    "Cisco Umbrella(OpenDNS)" => ("208.67.222.222,208.67.220.220", "2620:119:35::35,2620:119:53::53", Some("dns.opendns.com")),
    "Cloudflare" => ("1.1.1.1,1.0.0.1", "2606:4700:4700::1111,2606:4700:4700::1001", Some("cloudflare-dns.com")),
    "Cloudflare Malware and adult content blocking" => ("1.1.1.3,1.0.0.3", "2606:4700:4700::1113,2606:4700:4700::1003", Some("family.cloudflare-dns.com")),
    "Cloudflare Malware blocking" => ("1.1.1.2,1.0.0.2", "2606:4700:4700::1112,2606:4700:4700::1002", Some("security.cloudflare-dns.com")),
    "DNS.Watch" => ("84.200.69.80,84.200.70.40", "2001:1608:10:25::1c04:b12f,2001:1608:10:25::9249:d69b", None),
    "FFMUC DNS / Freie Netze Muenchen e.V." => ("185.150.99.255,5.1.66.255", "2001:678:e68:f000::,2001:678:ed0:f000::", Some("dot.ffmuc.net")),
    "GCore" => ("95.85.95.85,2.56.220.2", "2a03:90c0:999d::1,2a03:90c0:9992::1", None),
    "Google" => ("8.8.8.8,8.8.4.4", "2001:4860:4860::8888,2001:4860:4860::8844", Some("dns.google")),
    "Quad9" => ("9.9.9.9,149.112.112.112", "2620:fe::fe,2620:fe::9", Some("dns.quad9.net")),
    "Yandex" => ("77.88.8.8,77.88.8.1", "2a02:6b8::feed:0ff,2a02:6b8:0:1::feed:0ff", Some("common.dot.dns.yandex.net")),
    "Yandex Malware and adult content blocking" => ("77.88.8.7,77.88.8.3", "2a02:6b8::feed:a11,2a02:6b8:0:1::feed:a11", Some("family.dot.dns.yandex.net")),
    "Yandex Malware blocking" => ("77.88.8.88,77.88.8.2", "2a02:6b8::feed:bad,2a02:6b8:0:1::feed:bad", Some("safe.dot.dns.yandex.net")),
    "阿里云公共DNS (AliDNS)" => ("223.5.5.5,223.6.6.6", "2400:3200::1,2400:3200:baba::1", Some("dns.alidns.com")),
    "腾讯云 DNSPod (Tencent)" => ("119.29.29.29,119.28.28.28", "2402:4e00::,2402:4e00:1::", Some("dot.pub")),
};

pub static G_DNS_SERVER_INFO: phf::Map<&'static str, DnsServerInfo> = phf_map! {
    "AdGuard" => DnsServerInfo { region: "EU", homepage: "https://adguard-dns.io", is_filtering: false },
    "AdGuard Family Protection" => DnsServerInfo { region: "EU", homepage: "https://adguard-dns.io", is_filtering: true },
    "Cisco Umbrella(OpenDNS)" => DnsServerInfo { region: "US", homepage: "https://www.opendns.com", is_filtering: false },
    "Cloudflare" => DnsServerInfo { region: "US", homepage: "https://1.1.1.1", is_filtering: false },
    "Cloudflare Malware and adult content blocking" => DnsServerInfo { region: "US", homepage: "https://1.1.1.1/family", is_filtering: true },
    "Cloudflare Malware blocking" => DnsServerInfo { region: "US", homepage: "https://1.1.1.1/family", is_filtering: true },
    "DNS.Watch" => DnsServerInfo { region: "EU", homepage: "https://dns.watch", is_filtering: false },
    "FFMUC DNS / Freie Netze Muenchen e.V." => DnsServerInfo { region: "EU", homepage: "https://dns-setup.ffmuc.net", is_filtering: false },
    "GCore" => DnsServerInfo { region: "EU", homepage: "https://gcore.com/public-dns", is_filtering: false },
    "Google" => DnsServerInfo { region: "US", homepage: "https://dns.google", is_filtering: false },
    "Quad9" => DnsServerInfo { region: "EU", homepage: "https://www.quad9.net", is_filtering: false },
    "Yandex" => DnsServerInfo { region: "RU", homepage: "https://dns.yandex.com", is_filtering: false },
    "Yandex Malware and adult content blocking" => DnsServerInfo { region: "RU", homepage: "https://dns.yandex.com", is_filtering: true },
    "Yandex Malware blocking" => DnsServerInfo { region: "RU", homepage: "https://dns.yandex.com", is_filtering: true },
    "阿里云公共DNS (AliDNS)" => DnsServerInfo { region: "CN", homepage: "https://alidns.com", is_filtering: false },
    "腾讯云 DNSPod (Tencent)" => DnsServerInfo { region: "CN", homepage: "https://www.dnspod.cn", is_filtering: false },
};

#[derive(Subcommand, Debug)]
pub enum DnsAction {
    /// Set a DNS provider for a network connection
    Set {
        /// Network connection name (use 'list-connections' to see available)
        #[clap(short, long, value_name = "NAME")]
        connection: String,

        /// DNS provider to use (use 'list-servers' to see available)
        #[clap(short, long, value_enum)]
        server: DnsServer,

        /// Enable DNS over TLS (`DoT`) for the connection (requires server support)
        #[clap(long, conflicts_with_all = ["doh", "doq"])]
        dot: bool,

        /// Enable DNS over HTTPS (`DoH`) via blocky local proxy (requires server support)
        #[clap(long, conflicts_with_all = ["dot", "doq"])]
        doh: bool,

        /// Enable DNS over QUIC (`DoQ`) via blocky local proxy (requires server support)
        #[clap(long, conflicts_with_all = ["dot", "doh"])]
        doq: bool,
    },
    /// Set a custom DNS server for a network connection
    SetCustom {
        /// Network connection name (use 'list-connections' to see available)
        #[clap(short, long, value_name = "NAME")]
        connection: String,

        /// IPv4 DNS addresses (comma-separated, e.g. "1.1.1.1,1.0.0.1")
        #[clap(long, value_name = "ADDRS", default_value = "")]
        ipv4: String,

        /// IPv6 DNS addresses (comma-separated)
        #[clap(long, value_name = "ADDRS", default_value = "")]
        ipv6: String,

        /// Enable DNS over TLS (`DoT`)
        #[clap(long, conflicts_with_all = ["doh", "doq"])]
        dot: bool,

        /// `DoT` hostname for SNI (e.g. "dns.example.com")
        #[clap(long, value_name = "HOSTNAME", default_value = "")]
        dot_hostname: String,

        /// Enable DNS over HTTPS (`DoH`) via blocky local proxy
        #[clap(long, conflicts_with_all = ["dot", "doq"])]
        doh: bool,

        /// `DoH` URL (e.g. "<https://dns.example.com/dns-query>")
        #[clap(long, value_name = "URL", default_value = "")]
        doh_url: String,

        /// Enable DNS over QUIC (`DoQ`) via blocky local proxy
        #[clap(long, conflicts_with_all = ["dot", "doh"])]
        doq: bool,

        /// `DoQ` endpoint (e.g. "quic:dns.example.com")
        #[clap(long, value_name = "ENDPOINT", default_value = "")]
        doq_endpoint: String,
    },
    /// Reset DNS settings for a network connection to automatic (DHCP)
    Reset {
        /// Network connection name to reset
        #[clap(short, long, value_name = "NAME")]
        connection: String,
    },
    /// List available network connections managed by `NetworkManager`
    ListConnections,
    /// List available third-party DNS providers
    ListServers,
    /// Test latency to all DNS servers
    TestLatency,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DnsServer {
    AdGuard,
    AdGuardFamily,
    Cloudflare,
    CloudflareMalware,
    CloudflareMalwareAdult,
    OpenDns,
    DnsWatch,
    FFmuc,
    GCore,
    Google,
    Quad9,
    Yandex,
    YandexMalware,
    YandexMalwareAdult,
    AliDns,
    Tencent,
}

// TODO(vnepogodin): use these mapping instead of phf::map
impl DnsServer {
    pub fn as_str(&self) -> &'static str {
        match self {
            DnsServer::AdGuard => "AdGuard",
            DnsServer::AdGuardFamily => "AdGuard Family Protection",
            DnsServer::Cloudflare => "Cloudflare",
            DnsServer::CloudflareMalware => "Cloudflare Malware blocking",
            DnsServer::CloudflareMalwareAdult => "Cloudflare Malware and adult content blocking",
            DnsServer::OpenDns => "Cisco Umbrella(OpenDNS)",
            DnsServer::DnsWatch => "DNS.Watch",
            DnsServer::FFmuc => "FFMUC DNS / Freie Netze Muenchen e.V.",
            DnsServer::GCore => "GCore",
            DnsServer::Google => "Google",
            DnsServer::Quad9 => "Quad9",
            DnsServer::Yandex => "Yandex",
            DnsServer::YandexMalware => "Yandex Malware blocking",
            DnsServer::YandexMalwareAdult => "Yandex Malware and adult content blocking",
            DnsServer::AliDns => "阿里云公共DNS (AliDNS)",
            DnsServer::Tencent => "腾讯云 DNSPod (Tencent)",
        }
    }
}

/// DNS transport a connection can use. Maps to/from the dropdown's `active-id` string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsType {
    Plain,
    Dot,
    Doh,
    Doq,
}

impl DnsType {
    pub fn as_id(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Dot => "dot",
            Self::Doh => "doh",
            Self::Doq => "doq",
        }
    }

    /// Unknown or missing ids fall back to plain DNS.
    pub fn from_id(id: Option<&str>) -> Self {
        match id {
            Some("dot") => Self::Dot,
            Some("doh") => Self::Doh,
            Some("doq") => Self::Doq,
            _ => Self::Plain,
        }
    }
}

/// A [`DnsType`] paired with the hostname its transport needs.
#[derive(Debug, Clone)]
pub enum LatencyProbe {
    Plain,
    Dot { hostname: Arc<str> },
    Doh { hostname: Arc<str>, path: Option<Arc<str>> },
    Doq { hostname: Arc<str> },
}

impl LatencyProbe {
    /// Falls back to plain if `server_name` has no entry for `dns_type`.
    pub fn for_server(server_name: &str, dns_type: DnsType) -> Self {
        match dns_type {
            DnsType::Plain => Self::Plain,
            DnsType::Dot => G_DNS_SERVERS
                .get(server_name)
                .and_then(|&(_, _, hostname)| hostname)
                .map_or(Self::Plain, |hostname| Self::Dot { hostname: hostname.into() }),
            DnsType::Doh => G_DNS_DOH_URLS.get(server_name).map_or(Self::Plain, |&url| {
                let (hostname, path) = split_https_url(url);
                Self::Doh { hostname: hostname.into(), path }
            }),
            DnsType::Doq => G_DNS_DOQ_ENDPOINTS.get(server_name).map_or(Self::Plain, |&endpoint| {
                let hostname = endpoint.strip_prefix("quic:").unwrap_or(endpoint);
                Self::Doq { hostname: hostname.into() }
            }),
        }
    }
}

/// `https://host/path` -> (host, "/path"); no path yields `None`.
fn split_https_url(url: &str) -> (&str, Option<Arc<str>>) {
    let url = url.strip_prefix("https://").unwrap_or(url);
    match url.split_once('/') {
        Some((host, path)) => (host, Some(format!("/{path}").into())),
        None => (url, None),
    }
}

/// Times a real DNS lookup over `probe`'s transport, in milliseconds. `None` on timeout/error.
pub fn measure_latency(server_ipv4: &str, probe: &LatencyProbe) -> Option<u128> {
    use std::net::IpAddr;
    use std::time::{Duration, Instant};

    use hickory_resolver::Resolver;
    use hickory_resolver::config::{NameServerConfig, ResolveHosts, ResolverConfig, ResolverOpts};
    use hickory_resolver::net::runtime::TokioRuntimeProvider;

    // simple domain
    const PROBE_NAME: &str = "example.com.";

    let first_ip = server_ipv4.split(',').next()?;
    // Strip any #hostname suffix, then parse the bare IP.
    let ip: IpAddr = first_ip.split('#').next()?.trim().parse().ok()?;

    let name_server = match probe {
        LatencyProbe::Plain => NameServerConfig::udp(ip),
        LatencyProbe::Dot { hostname } => NameServerConfig::tls(ip, hostname.clone()),
        LatencyProbe::Doh { hostname, path } => {
            NameServerConfig::https(ip, hostname.clone(), path.clone())
        },
        LatencyProbe::Doq { hostname } => NameServerConfig::quic(ip, hostname.clone()),
    };
    let config = ResolverConfig::from_parts(None, vec![], vec![name_server]);
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(3);
    opts.attempts = 1;
    opts.cache_size = 0;
    // don't read /etc/hosts
    opts.use_hosts_file = ResolveHosts::Never;

    // each call already has its own worker thread
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().ok()?;
    rt.block_on(async move {
        let resolver = Resolver::builder_with_config(config, TokioRuntimeProvider::default())
            .with_options(opts)
            .build()
            .ok()?;
        let start = Instant::now();
        resolver.lookup_ip(PROBE_NAME).await.ok()?;
        Some(start.elapsed().as_millis())
    })
}

/// Append `#hostname` to each comma-separated address.
pub fn append_dot_hostname(addrs: &str, hostname: &str) -> String {
    if addrs.is_empty() {
        return String::new();
    }
    addrs.split(',').map(|addr| format!("{addr}#{hostname}")).collect::<Vec<_>>().join(",")
}

/// Returns true if `hostname` is a valid SNI/DoT hostname
pub fn is_valid_dot_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    }
    for label in hostname.split('.') {
        if label.is_empty() || label.len() > 63 || label.starts_with('-') || label.ends_with('-') {
            return false;
        }
        if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }
    true
}

/// Returns true if the server is a filtering variant (malware/family blocking).
pub fn is_filtering_server(name: &str) -> bool {
    G_DNS_SERVER_INFO.get(name).is_some_and(|info| info.is_filtering)
}

/// Measure latency for all DNS servers. Returns a Vec of (name, `latency_ms` or None).
pub fn measure_all_latencies() -> Vec<(&'static str, Option<u128>)> {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    let mut count = 0;

    for (name, (ipv4, ..)) in G_DNS_SERVERS.entries() {
        let tx = tx.clone();
        let ipv4 = ipv4.to_string();
        let name: &'static str = name;
        thread::spawn(move || {
            // plain-UDP baseline
            let latency = measure_latency(&ipv4, &LatencyProbe::Plain);
            let _ = tx.send((name, latency));
        });
        count += 1;
    }

    let mut results = Vec::with_capacity(count);
    for _ in 0..count {
        if let Ok(result) = rx.recv() {
            results.push(result);
        }
    }
    // Sort by latency (None/timeouts last)
    results.sort_by_key(|(_, ms)| ms.unwrap_or(u128::MAX));
    results
}

/// Encrypted DNS mode used by the blocky local proxy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockyMode {
    Doh,
    Doq,
}

fn blocky_preset_map(mode: BlockyMode) -> &'static phf::Map<&'static str, &'static str> {
    match mode {
        BlockyMode::Doh => &G_DNS_DOH_URLS,
        BlockyMode::Doq => &G_DNS_DOQ_ENDPOINTS,
    }
}

/// Returns the blocky upstream for a preset server, if supported.
pub fn get_blocky_upstream(server_name: &str, mode: BlockyMode) -> Option<&'static str> {
    blocky_preset_map(mode).get(server_name).copied()
}

pub fn is_valid_doh_url(url: &str) -> bool {
    url.starts_with("https://")
}

pub fn is_valid_doq_endpoint(endpoint: &str) -> bool {
    endpoint.starts_with("quic:") || endpoint.starts_with("quic://")
}

pub fn blocky_mode_from_upstream(upstream: &str) -> Option<BlockyMode> {
    if is_valid_doh_url(upstream) {
        Some(BlockyMode::Doh)
    } else if is_valid_doq_endpoint(upstream) {
        Some(BlockyMode::Doq)
    } else {
        None
    }
}

pub const BLOCKY_CONFIG_PATH: &str = "/etc/blocky/blocky.yml";
pub const BLOCKY_SERVICE: &str = "blocky.service";

/// Generate a blocky blocky.yml with bootstrap DNS.
/// `upstream` is e.g. "<https://cloudflare-dns.com/dns-query>" or "quic:cloudflare-dns.com"
/// `bootstrap_ipv4` is the plaintext IPv4 IPs, e.g. "1.1.1.1,1.0.0.1"
/// `bootstrap_ipv6` is the plaintext IPv6 IPs, e.g. "`2606:4700:4700::1111,2606:4700:4700::1001`"
/// `dot_hostname` is the optional `DoT` hostname — if provided, bootstrap uses `DoT` instead of
/// plaintext.
pub fn generate_blocky_config(
    upstream: &str,
    bootstrap_ipv4: &str,
    bootstrap_ipv6: &str,
    dot_hostname: Option<&str>,
) -> String {
    // Collect all bootstrap IPs (v4 + v6), filtering out empty strings
    let mut all_ips: Vec<&str> = bootstrap_ipv4.split(',').filter(|s| !s.is_empty()).collect();
    all_ips.extend(bootstrap_ipv6.split(',').filter(|s| !s.is_empty()));

    let bootstrap_section = match dot_hostname {
        Some(host) => {
            let ips =
                all_ips.iter().map(|ip| format!("      - \"{ip}\"")).collect::<Vec<_>>().join("\n");
            format!("  - upstream: tcp-tls:{host}\n    ips:\n{ips}")
        },
        None => all_ips
            .iter()
            .map(|ip| format!("  - upstream: \"{ip}\""))
            .collect::<Vec<_>>()
            .join("\n"),
    };

    format!(
        r#"# Generated by CachyOS Hello — do not edit manually
upstreams:
  groups:
    default:
      - "{upstream}"
  strategy: strict
  timeout: 5s
  userAgent: "CachyOS/blocky"

bootstrapDns:
{bootstrap_section}

ports:
  dns:
    - 127.0.0.1:53
    - "[::1]:53"

caching:
  minTime: 5m
  maxTime: 30m
  prefetching: true
"#
    )
}

/// Read the active blocky upstream from our generated config file, if present.
pub fn read_active_blocky_upstream() -> Option<String> {
    let config = std::fs::read_to_string(BLOCKY_CONFIG_PATH).ok()?;
    if !config.starts_with("# Generated by CachyOS Hello") {
        return None;
    }
    for line in config.lines() {
        let trimmed = line.trim().trim_start_matches("- ").trim_matches('"');
        if trimmed.starts_with("https://")
            || trimmed.starts_with("quic:")
            || trimmed.starts_with("quic://")
        {
            return Some(trimmed.to_string());
        }
    }
    None
}

/// Read active blocky mode and upstream from our generated config, if present.
pub fn read_active_blocky() -> Option<(BlockyMode, String)> {
    let upstream = read_active_blocky_upstream()?;
    let mode = blocky_mode_from_upstream(&upstream)?;
    Some((mode, upstream))
}

/// Given an active blocky upstream, find which preset server it belongs to.
pub fn find_server_by_blocky_upstream(upstream: &str, mode: BlockyMode) -> Option<usize> {
    for (idx, (name, _)) in G_DNS_SERVERS.entries().enumerate() {
        if blocky_preset_map(mode).get(name).is_some_and(|preset| *preset == upstream) {
            return Some(idx);
        }
    }
    None
}

/// Read bootstrap DNS info from blocky's config.
/// Returns (`ipv4_addrs`, `ipv6_addrs`, `dot_hostname`) parsed from the bootstrapDns section.
pub fn read_blocky_bootstrap() -> (String, String, Option<String>) {
    let config = match std::fs::read_to_string(BLOCKY_CONFIG_PATH) {
        Ok(c) => c,
        Err(_) => return (String::new(), String::new(), None),
    };

    let mut dot_hostname: Option<String> = None;
    let mut ipv4_addrs = Vec::new();
    let mut ipv6_addrs = Vec::new();
    let mut in_bootstrap = false;
    let mut in_ips = false;

    for line in config.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("bootstrapDns:") {
            in_bootstrap = true;
            continue;
        }
        if in_bootstrap
            && !trimmed.is_empty()
            && !trimmed.starts_with('-')
            && !trimmed.starts_with("ips:")
            && !line.starts_with(' ')
        {
            // Left the bootstrap section
            break;
        }
        if !in_bootstrap {
            continue;
        }
        // Parse "- upstream: tcp-tls:hostname" for DoT hostname
        if trimmed.starts_with("- upstream: tcp-tls:") {
            let host = trimmed.trim_start_matches("- upstream: tcp-tls:");
            dot_hostname = Some(host.to_string());
            in_ips = false;
            continue;
        }
        // Parse plaintext bootstrap "- upstream: \"IP\"" or "- upstream: IP"
        if trimmed.starts_with("- upstream:") && !trimmed.contains("tcp-tls") {
            let ip = trimmed.trim_start_matches("- upstream:").trim().trim_matches('"');
            if !ip.is_empty() {
                if ip.contains(':') {
                    ipv6_addrs.push(ip.to_string());
                } else {
                    ipv4_addrs.push(ip.to_string());
                }
            }
            continue;
        }
        if trimmed == "ips:" {
            in_ips = true;
            continue;
        }
        // Parse IP entries under ips:
        if in_ips && trimmed.starts_with("- ") {
            let ip = trimmed.trim_start_matches("- ").trim_matches('"');
            if ip.contains(':') {
                ipv6_addrs.push(ip.to_string());
            } else if !ip.is_empty() {
                ipv4_addrs.push(ip.to_string());
            }
        }
    }

    (ipv4_addrs.join(","), ipv6_addrs.join(","), dot_hostname)
}
