use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use super::get_type_str;

const ENDPOINT_URL: &str = "https://api.cloudflare.com/client/v4";

#[derive(Deserialize, Debug)]
struct Zone {
    id: String,
}

#[derive(Deserialize, Debug)]
struct Record {
    id: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct GetZone {
    success: bool,
    result: Vec<Zone>,
}
#[derive(Deserialize, Debug)]
struct RecordsResp {
    success: bool,
    result: Vec<Record>,
}
#[derive(Deserialize, Debug)]
struct RecordResp {
    success: bool,
    result: Record,
}

#[derive(Serialize, Debug)]
struct DnsReq {
    #[serde(rename = "type")]
    type_: String,
    name: String,
    content: String,
    ttl: usize,
}

fn get_zones(api_key: &str, zone_name: &str) -> Result<Vec<Zone>, crate::Error> {
    let resp: GetZone = reqwest::blocking::Client::new()
        .get(format!("{}/zones", ENDPOINT_URL))
        .bearer_auth(api_key)
        .query(&[("name", zone_name), ("status", "active")])
        .send()?
        .error_for_status()?
        .json()?;

    if !resp.success {
        return Err(crate::Error::ApiError);
    }

    Ok(resp.result)
}

fn get_records(
    api_key: &str,
    zone_id: &str,
    full_domain: &str,
) -> Result<Vec<Record>, crate::Error> {
    let resp: RecordsResp = reqwest::blocking::Client::new()
        .get(format!("{}/zones/{}/dns_records", ENDPOINT_URL, zone_id))
        .bearer_auth(api_key)
        .query(&[("name", full_domain)])
        .send()?
        .error_for_status()?
        .json()?;

    if !resp.success {
        return Err(crate::Error::ApiError);
    }

    Ok(resp.result)
}

fn update_record(
    api_key: &str,
    zone_id: &str,
    full_domain: &str,
    ip: &IpAddr,
    is_update: Option<&str>,
) -> Result<Record, crate::Error> {
    let req = DnsReq {
        type_: get_type_str(ip).to_owned(),
        name: full_domain.to_owned(),
        content: ip.to_string(),
        ttl: 1,
    };
    let resp = reqwest::blocking::Client::new();
    let resp = if let Some(recoard_key) = is_update {
        resp.put(format!(
            "{}/zones/{}/dns_records/{}",
            ENDPOINT_URL, zone_id, recoard_key
        ))
    } else {
        resp.post(format!("{}/zones/{}/dns_records", ENDPOINT_URL, zone_id))
    };

    let resp: RecordResp = resp
        .bearer_auth(api_key)
        .json(&req)
        .send()?
        .error_for_status()?
        .json()?;

    if !resp.success {
        return Err(crate::Error::ApiError);
    }

    Ok(resp.result)
}

pub fn update(
    api_key: &str,
    ip: &IpAddr,
    sub_domain: &str,
    apex_domain: &str,
) -> Result<IpAddr, crate::Error> {
    let full_domain = format!("{}.{}", sub_domain, apex_domain);

    let zones = get_zones(api_key, apex_domain)?;
    let zone = zones.get(0).ok_or(crate::Error::Unavailable)?;

    let records = get_records(api_key, &zone.id, &full_domain)?;
    let record = if let Some(record) = records.get(0) {
        update_record(
            api_key,
            &zone.id,
            &full_domain,
            ip,
            Some(record.id.as_str()),
        )?
    } else {
        update_record(api_key, &zone.id, &full_domain, ip, None)?
    };
    Ok(record.content.parse()?)
}
