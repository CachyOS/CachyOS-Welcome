use clap::{Subcommand, ValueEnum};
use phf::{phf_map, phf_ordered_map};

/// DNS server entry: (IPv4 addresses, IPv6 addresses, optional DoT hostname)
pub type DnsEntry = (&'static str, &'static str, Option<&'static str>);

/// Extra metadata for each DNS server (region, homepage URL, filtering variant).
pub struct DnsServerInfo {
    pub region: &'static str,
    pub homepage: &'static str,
    pub is_filtering: bool,
}

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

        /// Enable DNS over TLS (DoT) for the connection (requires server support)
        #[clap(long)]
        dot: bool,
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

        /// Enable DNS over TLS (DoT)
        #[clap(long)]
        dot: bool,

        /// DoT hostname for SNI (e.g. "dns.example.com")
        #[clap(long, value_name = "HOSTNAME", default_value = "")]
        dot_hostname: String,
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

/// Measure TCP connect latency to the first IPv4 address of a DNS server on port 53.
/// Returns the round-trip time in milliseconds, or None on timeout/error.
pub fn measure_latency(server_ipv4: &str) -> Option<u128> {
    use std::net::{SocketAddr, TcpStream};
    use std::time::{Duration, Instant};

    let first_ip = server_ipv4.split(',').next()?;
    // Strip any #hostname suffix for latency testing
    let ip_only = first_ip.split('#').next()?;
    let addr: SocketAddr = format!("{ip_only}:53").parse().ok()?;
    let start = Instant::now();
    TcpStream::connect_timeout(&addr, Duration::from_secs(3)).ok()?;
    Some(start.elapsed().as_millis())
}

/// Append `#hostname` to each comma-separated address.
pub fn append_dot_hostname(addrs: &str, hostname: &str) -> String {
    if addrs.is_empty() {
        return String::new();
    }
    addrs
        .split(',')
        .map(|addr| format!("{addr}#{hostname}"))
        .collect::<Vec<_>>()
        .join(",")
}

/// Returns true if `hostname` is a valid SNI/DoT hostname.
/// Must be non-empty, contain only alphanumeric chars, hyphens, and dots,
/// must not start/end with a dot or hyphen, and labels must be <= 63 chars.
pub fn is_valid_dot_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    }
    for label in hostname.split('.') {
        if label.is_empty()
            || label.len() > 63
            || label.starts_with('-')
            || label.ends_with('-')
        {
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

/// Measure latency for all DNS servers. Returns a Vec of (name, latency_ms or None).
pub fn measure_all_latencies() -> Vec<(&'static str, Option<u128>)> {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    let mut count = 0;

    for (name, (ipv4, _, _)) in G_DNS_SERVERS.entries() {
        let tx = tx.clone();
        let ipv4 = ipv4.to_string();
        let name: &'static str = name;
        thread::spawn(move || {
            let latency = measure_latency(&ipv4);
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