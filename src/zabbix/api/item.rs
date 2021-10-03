use serde_json::json;

use crate::zabbix::api::JSONRPC_VER;

use super::Api;

impl Api {
    pub fn item_get(session_id: &str, key: &str) -> serde_json::Value {
        json!({
            "jsonrpc": JSONRPC_VER,
            "method": "item.get",
            "params": {
                "output": "extend",
                "search": {
                    "key_": key
                }
            },
            "auth": session_id,
            "id": Self::gen_id()
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::zabbix::api::tests::TestAll;

    use super::*;

    #[tokio::test]
    async fn test_item_get() {
        let session_id = TestAll::setup().await.unwrap();

        // item get
        let key = "system";
        let request = Api::item_get(&session_id, key);
        let response = Api::post(TestAll::URL, request).await.unwrap();
        assert!(response["result"].as_array().is_some());
        // println!(
        //     "{}",
        //     serde_json::to_string_pretty(&response["result"]).unwrap()
        // );

        // logout
        TestAll::teardown(&session_id).await.unwrap();
    }
}
