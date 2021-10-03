pub mod item;
pub mod trend;
pub mod user;

use anyhow::Error;
use rand::Rng;

pub const JSONRPC_VER: &str = "2.0";

pub struct Api {}

impl Api {
    pub async fn post(url: &str, json: serde_json::Value) -> Result<serde_json::Value, Error> {
        let json: serde_json::Value = reqwest::Client::new()
            .post(url)
            .json(&json)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(json)
    }

    fn gen_id() -> i32 {
        rand::thread_rng().gen()
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use anyhow::anyhow;

    pub struct TestAll {}

    impl TestAll {
        pub const URL: &'static str = "http://localhost/api_jsonrpc.php";

        pub async fn setup() -> Result<String, Error> {
            // login
            let username = "Admin";
            let password = "zabbix";
            let request = Api::user_login(username, password);
            let response = Api::post(Self::URL, request).await?;
            match response["result"].as_str() {
                Some(session_id) => Ok(session_id.to_owned()),
                _ => Err(anyhow!("login error")),
            }
        }

        pub async fn teardown(session_id: &str) -> Result<(), Error> {
            // logout
            let request = Api::user_logout(session_id);
            let response = Api::post(Self::URL, request).await.unwrap();
            match response["result"].as_bool() {
                Some(true) => Ok(()),
                _ => Err(anyhow!("logout error")),
            }
        }
    }
}
