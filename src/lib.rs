//! Official HS DEVS AUTH Rust Client SDK
//! High-performance, memory-safe licensing and user authentication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserData {
    pub username: String,
    pub ip: String,
    pub hwid: String,
    pub created_at: String,
    pub expires_at: String,
    pub role: String,
    pub level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
}

pub struct HSAuth {
    pub name: String,
    pub ownerid: String,
    pub secret: String,
    pub version: String,
    pub api_url: String,
    pub session_id: String,
    pub initialized: bool,
    pub user_data: UserData,
    pub response: AuthResponse,
}

impl HSAuth {
    pub fn new(name: &str, ownerid: &str, secret: &str, version: &str) -> Self {
        HSAuth {
            name: name.to_string(),
            ownerid: ownerid.to_string(),
            secret: secret.to_string(),
            version: version.to_string(),
            api_url: "https://auth.hsdevs.pro/api/1.3/".to_string(),
            session_id: String::new(),
            initialized: false,
            user_data: UserData::default(),
            response: AuthResponse::default(),
        }
    }

    pub fn init(&mut self) -> bool {
        let client = reqwest::blocking::Client::new();
        let mut params = HashMap::new();
        params.insert("type", "init");
        params.insert("name", &self.name);
        params.insert("ownerid", &self.ownerid);
        params.insert("ver", &self.version);

        match client.post(&self.api_url).form(&params).send() {
            Ok(res) => {
                if let Ok(json) = res.json::<serde_json::Value>() {
                    let success = json["success"].as_bool().unwrap_or(false);
                    let message = json["message"].as_str().unwrap_or("").to_string();
                    self.response = AuthResponse { success, message: message.clone() };
                    if success {
                        self.session_id = json["sessionid"].as_str().unwrap_or("").to_string();
                        self.initialized = true;
                        return true;
                    }
                }
            }
            Err(e) => {
                self.response = AuthResponse { success: false, message: e.to_string() };
            }
        }
        false
    }

    pub fn login(&mut self, username: &str, pass: &str) -> bool {
        if !self.initialized && !self.init() {
            return false;
        }

        let client = reqwest::blocking::Client::new();
        let mut params = HashMap::new();
        params.insert("type", "login");
        params.insert("username", username);
        params.insert("pass", pass);
        params.insert("sessionid", &self.session_id);
        params.insert("name", &self.name);
        params.insert("ownerid", &self.ownerid);

        match client.post(&self.api_url).form(&params).send() {
            Ok(res) => {
                if let Ok(json) = res.json::<serde_json::Value>() {
                    let success = json["success"].as_bool().unwrap_or(false);
                    let message = json["message"].as_str().unwrap_or("").to_string();
                    self.response = AuthResponse { success, message };
                    if success {
                        if let Some(info) = json.get("info") {
                            self.user_data.username = info["username"].as_str().unwrap_or(username).to_string();
                            self.user_data.ip = info["ip"].as_str().unwrap_or("").to_string();
                            self.user_data.hwid = info["hwid"].as_str().unwrap_or("").to_string();
                            self.user_data.created_at = info["createdate"].as_str().unwrap_or("").to_string();
                            self.user_data.expires_at = info["expiry"].as_str().unwrap_or("").to_string();
                        }
                        return true;
                    }
                }
            }
            Err(e) => {
                self.response = AuthResponse { success: false, message: e.to_string() };
            }
        }
        false
    }

    pub fn license(&mut self, key: &str) -> bool {
        if !self.initialized && !self.init() {
            return false;
        }

        let client = reqwest::blocking::Client::new();
        let mut params = HashMap::new();
        params.insert("type", "license");
        params.insert("key", key);
        params.insert("sessionid", &self.session_id);
        params.insert("name", &self.name);
        params.insert("ownerid", &self.ownerid);

        match client.post(&self.api_url).form(&params).send() {
            Ok(res) => {
                if let Ok(json) = res.json::<serde_json::Value>() {
                    let success = json["success"].as_bool().unwrap_or(false);
                    let message = json["message"].as_str().unwrap_or("").to_string();
                    self.response = AuthResponse { success, message };
                    if success {
                        if let Some(info) = json.get("info") {
                            self.user_data.username = info["username"].as_str().unwrap_or("").to_string();
                            self.user_data.expires_at = info["expiry"].as_str().unwrap_or("").to_string();
                        }
                        return true;
                    }
                }
            }
            Err(e) => {
                self.response = AuthResponse { success: false, message: e.to_string() };
            }
        }
        false
    }
}
