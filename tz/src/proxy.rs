#[derive(Clone, Copy)]
pub struct ProxyDef {
    pub id: &'static str,
    pub name: &'static str,
    pub flag: &'static str,
    pub port: u16,
}

pub const PROXIES: &[ProxyDef] = &[
    ProxyDef { id: "hk", name: "香港", flag: "🇭🇰", port: 1081 },
    ProxyDef { id: "jp", name: "日本", flag: "🇯🇵", port: 1082 },
    ProxyDef { id: "us", name: "美国", flag: "🇺🇸", port: 1083 },
    ProxyDef { id: "sg", name: "新加坡", flag: "🇸🇬", port: 1084 },
    ProxyDef { id: "va", name: "弗吉尼亚", flag: "🇺🇸", port: 1085 },
    ProxyDef { id: "jk", name: "家宽", flag: "🏠", port: 1086 },
];

pub const PROXY_HOST: &str = "127.0.0.1";
