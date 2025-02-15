use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::http_client::HTTPClient;

const API_URL: &str = "https://crates.io/api/v1/crates";
const API_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1000);

#[derive(Deserialize, Debug)]
pub struct CrateInfo {
    #[serde(rename = "crate")]
    _crate: Crate,
}

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub created_at: DateTime<Utc>,
    pub downloads: u64,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub repository: String,
    pub versions: Vec<u64>,
    #[serde(skip)]
    pub reverse_dependencies: u64,
    #[serde(skip)]
    pub contributors: u16,
}

#[derive(Deserialize, Debug)]
struct ReverseDependencies {
    meta: Meta,
}

#[derive(Deserialize, Debug)]
struct Meta {
    total: u64,
}

pub fn get_crate_info(client: &HTTPClient, crate_name: String) -> Result<Crate, Error> {
    std::thread::sleep(API_INTERVAL);

    let url = format!("{}/{}", API_URL, &crate_name);

    let mut crate_info: CrateInfo = serde_json::from_str(&client.get(&url)?)
        .with_context(|| format!("failed to deserialize response from: {}", url))?;

    // crates.io treats - and _ the same, set crate name to cargo tree name
    // so when appending we don't get the name again
    crate_info._crate.name = crate_name.clone();

    crate_info._crate.reverse_dependencies = get_reverse_dependencies(client, &crate_name)
        .with_context(|| format!("failed to get reverse dependencies for {}", crate_name))?;

    Ok(crate_info._crate)
}

fn get_reverse_dependencies(client: &HTTPClient, crate_name: &str) -> Result<u64, Error> {
    std::thread::sleep(API_INTERVAL);

    let url = format!("{}/{}/reverse_dependencies", API_URL, crate_name);

    let reverse_dependencies: ReverseDependencies = serde_json::from_str(&client.get(&url)?)
        .with_context(|| format!("failed to deserialize response from: {}", url))?;

    Ok(reverse_dependencies.meta.total)
}

