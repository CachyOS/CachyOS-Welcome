use clap::{Subcommand, ValueEnum};
use phf::phf_ordered_map;

pub static G_DNS_SERVERS: phf::OrderedMap<&'static str, (&'static str, &'static str)> = phf_ordered_map! {
    "AdGuard" => ("94.140.14.14,94.140.15.15", "2a10:50c0::ad1:ff,2a10:50c0::ad2:ff"),
    "AdGuard Family Protection" => ("94.140.14.15,94.140.15.16", "2a10:50c0::bad1:ff,2a10:50c0::bad2:ff"),
    "Cloudflare" => ("1.1.1.1,1.0.0.1", "2606:4700:4700::1111,2606:4700:4700::1001"),
    "Cloudflare Malware blocking" => ("1.1.1.2,1.0.0.2", "2606:4700:4700::1112,2606:4700:4700::1002"),
    "Cloudflare Malware and adult content blocking" => ("1.1.1.3,1.0.0.3", "2606:4700:4700::1113,2606:4700:4700::1003"),
    "Cisco Umbrella(OpenDNS)" => ("208.67.222.222,208.67.220.220", "2620:119:35::35,2620:119:53::53"),
    "DNS.Watch" => ("84.200.69.80,84.200.70.40", "2001:1608:10:25::1c04:b12f,2001:1608:10:25::9249:d69b"),
    "GCore" => ("95.85.95.85,2.56.220.2", "2a03:90c0:999d::1,2a03:90c0:9992::1"),
    "Google" => ("8.8.8.8,8.8.4.4", "2001:4860:4860::8888,2001:4860:4860::8844"),
    "Quad9" => ("9.9.9.9,149.112.112.112", "2620:fe::fe,2620:fe::9"),
    "Yandex" => ("77.88.8.8,77.88.8.1", "2a02:6b8::feed:0ff,2a02:6b8:0:1::feed:0ff"),
    "Yandex Malware blocking" => ("77.88.8.88,77.88.8.2", "2a02:6b8::feed:bad,2a02:6b8:0:1::feed:bad"),
    "Yandex Malware and adult content blocking" => ("77.88.8.7,77.88.8.3", "2a02:6b8::feed:a11,2a02:6b8:0:1::feed:a11"),
    "阿里云公共DNS (AliDNS)" => ("223.5.5.5,223.6.6.6", "2400:3200::1,2400:3200:baba::1"),
    "腾讯云 DNSPod (Tencent)" => ("119.29.29.29,119.28.28.28", "2402:4e00::,2402:4e00:1::"),
    "FFMUC DNS" => ("5.1.66.255,185.150.99.255", "2001:678:e68:f000::,2001:678:ed0:f000::")
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
    GCore,
    Google,
    Quad9,
    Yandex,
    YandexMalware,
    YandexMalwareAdult,
    AliDns,
    Tencent,
    FFMUC,
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
            DnsServer::GCore => "GCore",
            DnsServer::Google => "Google",
            DnsServer::Quad9 => "Quad9",
            DnsServer::Yandex => "Yandex",
            DnsServer::YandexMalware => "Yandex Malware blocking",
            DnsServer::YandexMalwareAdult => "Yandex Malware and adult content blocking",
            DnsServer::AliDns => "阿里云公共DNS (AliDNS)",
            DnsServer::Tencent => "腾讯云 DNSPod (Tencent)",
            DnsServer::FFMUC => "FFMUC DNS",
        }
    }
}
