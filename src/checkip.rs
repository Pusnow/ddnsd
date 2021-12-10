use std::net::IpAddr;

const CHECK_IP_URL: &str = "https://checkip.amazonaws.com";

pub fn check_ip() -> Result<IpAddr, crate::Error> {
    let resp = reqwest::blocking::get(CHECK_IP_URL)?.text()?;
    let ip: IpAddr = resp.as_str().trim().parse()?;
    Ok(ip)
}
