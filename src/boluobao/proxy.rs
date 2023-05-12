use super::consts;
use crate::Value;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use uuid::Uuid;

struct Config {
    version: String,
    channel: String,
    device_token: String,
}

pub struct ProxyBuilder {
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    version: String,
    channel: String,
    device_token: String,
    cache: Map<String, Value>,
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
                cache: Map::<String, Value>::new(),
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

    pub fn get_app_version(&self) -> &String {
        &self.version
    }

    pub fn get_channel(&self) -> &String {
        &self.channel
    }

    pub fn get_device_token(&self) -> &String {
        &self.device_token
    }

    pub fn is_cached(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    pub fn load(&self, key: &str) -> Option<&Value> {
        self.cache.get(key)
    }

    pub fn load_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.cache.get_mut(key)
    }

    pub fn store(&mut self, key: &str, value: Value) -> Option<Value> {
        self.cache.insert(key.to_string(), value)
    }
}
