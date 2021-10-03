use serde_json::json;

use crate::zabbix::api::JSONRPC_VER;

use super::Api;

impl Api {
    pub fn user_login(username: &str, password: &str) -> serde_json::Value {
        json!({
            "jsonrpc": JSONRPC_VER,
            "method": "user.login",
            "params": {
                "username": username,
                "password": password
            },
            "id": Self::gen_id()
        })
    }

    pub fn user_logout(session_id: &str) -> serde_json::Value {
        json!({
            "jsonrpc": JSONRPC_VER,
            "method": "user.logout",
            "params": [],
            "id": Self::gen_id(),
            "auth": session_id
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_login_logout() {
        let url = "http://localhost/api_jsonrpc.php";

        // login
        let username = "Admin";
        let password = "zabbix";
        let request = Api::user_login(username, password);
        // println!("{:?}", &request);
        let response = Api::post(url, request).await.unwrap();
        let session_id = response["result"].as_str();
        assert!(session_id.is_some());

        // logout
        let request = Api::user_logout(session_id.unwrap());
        // println!("{:?}", &request);
        let response = Api::post(url, request).await.unwrap();
        // println!("{:?}", &response);
        assert_eq!(response["result"].as_bool(), Some(true));
    }
}
