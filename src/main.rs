use clap::{value_t_or_exit, App, Arg};
use std::thread::sleep;
use std::time::Duration;

use log::info;

mod checkip;
mod provider;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Parse(std::net::AddrParseError),
    ApiError,
    Unavailable,
}
impl From<reqwest::Error> for Error {
    fn from(s: reqwest::Error) -> Error {
        Error::Request(s)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(s: std::net::AddrParseError) -> Error {
        Error::Parse(s)
    }
}

fn main() -> Result<(), Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("provider")
                .short('p')
                .long("provider")
                .required(true)
                .takes_value(true)
                .possible_values(&provider::Provider::variants())
                .case_insensitive(true)
                .env("DDNSD_PROVIDER")
                .help("API key"),
        )
        .arg(
            Arg::with_name("key")
                .short('k')
                .long("key")
                .required(true)
                .takes_value(true)
                .env("DDNSD_KEY")
                .help("API key"),
        )
        .arg(
            Arg::with_name("duration")
                .short('d')
                .long("duration")
                .default_value("600")
                .takes_value(true)
                .env("DDNSD_DURATION")
                .help("Renewal duration in seconds"),
        )
        .arg(
            Arg::with_name("sub")
                .short('s')
                .long("sub")
                .required(true)
                .takes_value(true)
                .env("DDNSD_SUB")
                .help("Sub domain"),
        )
        .arg(
            Arg::with_name("apex")
                .short('a')
                .long("apex")
                .required(true)
                .takes_value(true)
                .env("DDNSD_APEX")
                .help("Apex domain"),
        )
        .get_matches();

    let provider = value_t_or_exit!(matches, "provider", provider::Provider);
    let duration = value_t_or_exit!(matches, "duration", u64);

    let key = matches.value_of("key").unwrap();
    let sub = matches.value_of("sub").unwrap();
    let apex = matches.value_of("apex").unwrap();
    env_logger::init();
    info!("provider: {}", provider);
    info!("duration: {}", duration);
    info!("sub: {}", sub);
    info!("apex: {}", apex);

    let mut ip_prev = None;

    info!("DDNSD started");
    loop {
        let ip_current = checkip::check_ip()?;
        info!("Current IP: {} Previous IP: {:?}", ip_current, ip_prev);
        if ip_prev != Some(ip_current) {
            info!("Update DDNS IP to {}", ip_current);
            provider::update(&provider, key, &ip_current, sub, apex)?;
        }
        ip_prev = Some(ip_current);
        sleep(Duration::from_secs(duration));
    }
}
