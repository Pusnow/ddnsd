use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use super::get_type_str;

const ENDPOINT_URL: &str = "https://api.digitalocean.com/v2/domains";

#[derive(Deserialize, Serialize, Debug)]
struct Record {
    id: u64,
    #[serde(rename = "type")]
    type_: String,
    name: String,
    data: String,
}

#[derive(Deserialize, Debug)]
struct Resps {
    domain_records: Vec<Record>,
}

#[derive(Deserialize, Debug)]
struct Resp {
    domain_record: Record,
}

fn do_get(
    api_key: &str,
    endpoint: &str,
    full_domain: &str,
    type_: &str,
) -> Result<Vec<Record>, reqwest::Error> {
    let resps: Resps = reqwest::blocking::Client::new()
        .get(endpoint)
        .bearer_auth(api_key)
        .query(&[("name", full_domain), ("type", type_)])
        .send()?
        .error_for_status()?
        .json()?;
    Ok(resps.domain_records)
}

fn do_create(
    api_key: &str,
    endpoint: &str,
    sub_domain: &str,
    type_: &str,
    data: &str,
) -> Result<Record, reqwest::Error> {
    let record = Record {
        id: 0,
        type_: type_.to_string(),
        name: sub_domain.to_string(),
        data: data.to_string(),
    };
    let resp: Resp = reqwest::blocking::Client::new()
        .post(endpoint)
        .bearer_auth(api_key)
        .json(&record)
        .send()?
        .error_for_status()?
        .json()?;
    Ok(resp.domain_record)
}
fn do_update(
    api_key: &str,
    endpoint: &str,
    sub_domain: &str,
    id: u64,
    type_: &str,
    data: &str,
) -> Result<Record, reqwest::Error> {
    let record = Record {
        id,
        type_: type_.to_string(),
        name: sub_domain.to_string(),
        data: data.to_string(),
    };
    let resp: Resp = reqwest::blocking::Client::new()
        .put(format!("{}/{}", endpoint, id))
        .bearer_auth(api_key)
        .json(&record)
        .send()?
        .error_for_status()?
        .json()?;
    Ok(resp.domain_record)
}

pub fn update(
    api_key: &str,
    ip: &IpAddr,
    sub_domain: &str,
    apex_domain: &str,
) -> Result<IpAddr, crate::Error> {
    let endpoint = format!("{}/{}/records", ENDPOINT_URL, apex_domain);
    let full_domain = format!("{}.{}", sub_domain, apex_domain);
    let type_str = get_type_str(ip);

    let records = do_get(api_key, &endpoint, &full_domain, type_str)?;

    let record = if records.is_empty() {
        do_create(
            api_key,
            &endpoint,
            sub_domain,
            type_str,
            ip.to_string().as_ref(),
        )?
    } else {
        let id = records[0].id;
        do_update(
            api_key,
            &endpoint,
            sub_domain,
            id,
            type_str,
            ip.to_string().as_ref(),
        )?
    };

    Ok(record.data.parse()?)
}
