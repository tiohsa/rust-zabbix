use chrono::NaiveDateTime;
use serde_json::json;

use crate::zabbix::api::JSONRPC_VER;

use super::Api;

impl Api {
    pub fn trend_get(
        session_id: &str,
        itemids: &Vec<String>,
        time_from: &NaiveDateTime,
        time_til: &NaiveDateTime,
    ) -> serde_json::Value {
        json!({
            "jsonrpc": JSONRPC_VER,
            "method": "trend.get",
            "params": {
                "output": [
                    "itemid",
                    "clock",
                    "num",
                    "value_min",
                    "value_avg",
                    "value_max",
                ],
                "itemids": itemids,
                "time_from": time_from.timestamp(),
                "time_til": time_til.timestamp(),
            },
            "auth": session_id,
            "id": Self::gen_id()
        })
    }
}

#[cfg(test)]
mod tests {

    use chrono::{Duration, Local};

    use crate::zabbix::api::tests::TestAll;

    use super::*;

    #[tokio::test]
    async fn test_trend_get() {
        let session_id = TestAll::setup().await.unwrap();

        // trend get
        let items = vec!["10078", "10073"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let time_til = Local::now().naive_local();
        let time_from = time_til - Duration::days(765);
        let request = Api::trend_get(&session_id, &items, &time_from, &time_til);
        println!("{}", serde_json::to_string_pretty(&request).unwrap());
        let response = Api::post(TestAll::URL, request).await.unwrap();

        assert!(response["result"].as_array().is_some());
        println!(
            "{}",
            serde_json::to_string_pretty(&response["result"]).unwrap()
        );

        // logout
        TestAll::teardown(&session_id).await.unwrap();
    }
}
