use anyhow::{Error, Result};
use dateparser::DateTimeUtc;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_CHARSET, AUTHORIZATION, CONTENT_TYPE, SET_COOKIE, USER_AGENT,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

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
    cache: serde_json::Value,
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
                cache: json!({}),
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

impl Proxy {
    pub fn is_authenticated(&self) -> bool {
        if let Some(value) = self.cache.get("auth") {
            let map = value.as_object().unwrap();
            if !(map.contains_key(".SFCommunity") && map.contains_key("session_APP")) {
                return false;
            }
            let expires = map.get("expires").unwrap().as_u64().unwrap();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            expires > timestamp
        } else {
            false
        }
    }

    pub fn login(&mut self, account: &str, password: &str) -> Option<String> {
        if self.is_authenticated() {
            return Some("Authentication is already done".to_string());
        }

        let client = Client::new();

        let url = format!("{}/sessions", consts::APIPREFIX);
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
                Some(status.get("msg").unwrap().as_str().unwrap().to_string())
            } else {
                Some(resp.to_string())
            };
        }

        let re = Regex::new(r"^(?<key>[^=]+)=(?<value>[^;]+).*expires=(?<expires>[^;]+)").unwrap();
        let cookies: HashMap<&str, (u64, &str)> =
            HashMap::from_iter(resp.headers().get_all(SET_COOKIE).iter().map(|value| {
                let result = re.captures(value.to_str().unwrap()).unwrap();
                let expires = result
                    .name("expires")
                    .unwrap()
                    .as_str()
                    .replace("-", " ")
                    .parse::<DateTimeUtc>()
                    .unwrap()
                    .0
                    .timestamp() as u64;
                (
                    result.name("key").unwrap().as_str(),
                    (expires, result.name("value").unwrap().as_str()),
                )
            }));

        let mut auth: HashMap<String, String> = HashMap::new();
        let mut auth_expires = u64::MAX;
        for key in vec![".SFCommunity", "session_APP"] {
            assert!(cookies.contains_key(key));
            let (expires, value) = cookies.get(key).unwrap();
            if expires < &auth_expires {
                auth_expires = expires.clone();
            }
            auth.insert(key.to_string(), value.to_string());
        }
        auth.insert("account".to_string(), account.to_string());
        auth.insert("password".to_string(), password.to_string());

        let mut auth = Value::from_iter(auth.iter().map(|(k, v)| (k.to_owned(), v.to_owned())));
        auth.as_object_mut()
            .unwrap()
            .insert("expires".to_string(), json!(auth_expires));
        self.cache
            .as_object_mut()
            .unwrap()
            .insert("auth".to_string(), auth);

        None
    }

    pub fn logout(&mut self) -> bool {
        let authenticated = self.is_authenticated();
        self.cache.as_object_mut().unwrap().remove("auth");
        authenticated
    }
}
