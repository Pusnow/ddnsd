pub mod cloudflare;
pub mod digitaloceon;
use clap::arg_enum;
use std::net::IpAddr;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Provider {
        DigitalOceon,
        Cloudflare,
    }
}

fn get_type_str(ip: &IpAddr) -> &'static str {
    match ip {
        IpAddr::V4(_) => "A",
        IpAddr::V6(_) => "AAAA",
    }
}

pub fn update(
    provider: &Provider,
    api_key: &str,
    ip: &IpAddr,
    sub_domain: &str,
    apex_domain: &str,
) -> Result<IpAddr, crate::Error> {
    match provider {
        Provider::DigitalOceon => digitaloceon::update(api_key, ip, sub_domain, apex_domain),
        Provider::Cloudflare => cloudflare::update(api_key, ip, sub_domain, apex_domain),
    }
}
