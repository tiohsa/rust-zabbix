use std::fs::File;

use anyhow::{anyhow, Error};
use chrono::{Duration, Local, NaiveDateTime};
use rust_zabbix::zabbix::api::Api;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct ZabbixServer {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

struct ZabbixData {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

async fn user_login(server: &ZabbixServer) -> Result<String, Error> {
    let request = Api::user_login(&server.username, &server.password);
    let response = Api::post(&server.url, request).await?;
    match response["result"].as_str() {
        Some(session_id) => Ok(session_id.to_owned()),
        _ => Err(anyhow!("login error")),
    }
}

async fn user_logout(server: &ZabbixServer, session_id: &str) -> Result<(), Error> {
    let request = Api::user_logout(session_id);
    let response = Api::post(&server.url, request).await?;
    match response["result"].as_bool() {
        Some(true) => Ok(()),
        _ => Err(anyhow!("logout error")),
    }
}

async fn item_get(
    server: &ZabbixServer,
    session_id: &str,
    key: &str,
) -> Result<serde_json::Value, Error> {
    let request = Api::item_get(session_id, key);
    let response = Api::post(&server.url, request).await?;
    Ok(response)
}

async fn trend_get(
    server: &ZabbixServer,
    session_id: &str,
    itemids: &Vec<String>,
    time_from: &NaiveDateTime,
    time_til: &NaiveDateTime,
) -> Result<serde_json::Value, Error> {
    let request = Api::trend_get(session_id, itemids, time_from, time_til);
    let response = Api::post(&server.url, request).await?;
    Ok(response)
}

async fn save_trend_data(server: &ZabbixServer) -> Result<(), Error> {
    let session_id = user_login(server).await?;

    let result = get_trend_data(server, &session_id).await;

    user_logout(server, &session_id).await?;

    result?;
    Ok(())
}

async fn get_trend_data(server: &ZabbixServer, session_id: &str) -> Result<(), Error> {
    let key = "cpu";
    let response = item_get(server, &session_id, &key).await?;
    let itemids = if let Some(items) = response["result"].as_array() {
        items
            .iter()
            .flat_map(|item| item["itemid"].as_str())
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let time_til = Local::now().naive_local();
    let time_from = time_til - Duration::days(765);
    trend_get(server, &session_id, &itemids, &time_from, &time_til).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let zabbix_server_list = "zabbix_server_list.csv";
    let mut rdr = csv::Reader::from_reader(File::open(zabbix_server_list)?);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let server: ZabbixServer = result?;
        println!("{:?}", server);
        save_trend_data(&server).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
}
