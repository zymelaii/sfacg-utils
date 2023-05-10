use std::collections::HashMap;

use anyhow::{Error, Ok, Result};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_CHARSET, AUTHORIZATION, CONTENT_TYPE, SET_COOKIE, USER_AGENT,
};
use serde_json::json;
pub use uuid::Uuid;

use super::api::*;
use super::consts;
use super::encrypt::*;

struct Config {
    version: String,
    channel: String,
    device_token: String,
}

pub struct ProxyBuilder {
    config: Config,
}

#[derive(Debug)]
pub struct Proxy {
    version: String,
    channel: String,
    device_token: String,
    cache: HashMap<String, String>,
}

impl ProxyBuilder {
    fn new() -> Self {
        let version = consts::APPKEYS.keys().next().unwrap().to_string();
        let device_token = Uuid::new_v4().to_string().to_lowercase();
        Self {
            config: Config {
                version: version,
                channel: "HomePage".to_string(),
                device_token: device_token,
            },
        }
    }

    pub fn build(self) -> Result<Proxy> {
        let config = self.config;
        if consts::APPKEYS.contains_key(&config.version) {
            Ok(Proxy {
                version: config.version,
                channel: config.channel,
                device_token: config.device_token,
                cache: HashMap::new(),
            })
        } else {
            Err(Error::msg(format!(
                "invalid app version: {}",
                config.version
            )))
        }
    }

    pub fn with_app_version(mut self, version: &str) -> Self {
        self.config.version = version.to_string();
        self
    }

    pub fn with_channel(mut self, channel: &str) -> Self {
        self.config.channel = channel.to_string();
        self
    }

    pub fn with_device_token(mut self, device_token: Uuid) -> Self {
        self.config.device_token = device_token.to_string();
        self
    }
}

impl Proxy {
    pub fn builder() -> ProxyBuilder {
        ProxyBuilder::new()
    }

    pub fn default() -> Self {
        Self::builder().build().unwrap()
    }

    fn user_agent(&self) -> String {
        format!(
            "boluobao/{}/{}/{}",
            self.version, self.channel, self.device_token
        )
    }

    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let accept = "application/vnd.sfacg.api+json;version=1";
        let security = get_sfsecurity(&self.version, &self.device_token);

        headers.insert(ACCEPT, accept.parse().unwrap());
        headers.insert(ACCEPT_CHARSET, "UTF-8".parse().unwrap());
        headers.insert(AUTHORIZATION, consts::AUTH.parse().unwrap());
        headers.insert(USER_AGENT, self.user_agent().parse().unwrap());
        headers.insert("SFSecurity", security.parse().unwrap());

        headers
    }
}

impl AuthApi for Proxy {
    fn is_authenticated(&self) -> bool {
        self.cache.contains_key(".SFCommunity") && self.cache.contains_key("session_APP")
    }

    fn login(&mut self, account: &str, password: &str) -> Option<String> {
        if self.is_authenticated() {
            return Some("Authentication is already done".to_string());
        }

        let client = Client::new();

        let url = "https://api.sfacg.com/sessions";
        let secrets = json!({
            "username": account,
            "password": password,
        });

        let mut headers = self.default_headers();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let resp = client
            .post(url)
            .headers(headers)
            .body(secrets.to_string())
            .send()
            .unwrap();

        if resp.status() != 200 {
            let resp: serde_json::Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
            return if let Some(status) = resp.get("status") {
                Some(status.get("msg").unwrap().to_string())
            } else {
                Some(resp.to_string())
            }
        }

        let re = Regex::new(r"^(?<key>[^=]+)=(?<value>[^;]+)").unwrap();
        let cookies: HashMap<&str, &str> =
            HashMap::from_iter(resp.headers().get_all(SET_COOKIE).iter().map(|value| {
                let result = re.captures(value.to_str().unwrap()).unwrap();
                (
                    result.name("key").unwrap().as_str(),
                    result.name("value").unwrap().as_str(),
                )
            }));

        for key in vec![".SFCommunity", "session_APP"] {
            assert!(cookies.contains_key(key));
            let value = cookies.get(key).unwrap().to_string();
            self.cache.insert(key.to_string(), value);
        }

        None
    }

    fn logout(&mut self) -> bool {
        if self.is_authenticated() {
            self.cache.remove(".SFCommunity");
            self.cache.remove("session_APP");
            true
        } else {
            false
        }
    }
}
