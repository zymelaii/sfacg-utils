use super::types;
use crate::{request::*, Proxy, Value};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCredential {
    pub ident: Option<String>,
    pub token: String,
    pub session: String,
    pub secrets: (String, String),
    pub expires: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    pub status: (bool, usize, Option<String>),
    pub credentials: Vec<AuthCredential>,
}

impl Proxy {
    pub fn is_authenticated(&self) -> bool {
        if let Some(value) = self.load("auth") {
            let authentications = serde_json::from_value::<AuthStatus>(value.to_owned()).unwrap();
            let auths = &authentications.credentials;
            let (active, index, ident) = authentications.status;
            if !active || index >= auths.len() {
                return false;
            }
            let auth = &authentications.credentials[index];
            if auth.ident.is_some() && ident.is_some() {
                if auth.ident.as_ref().unwrap().to_owned() != ident.unwrap() {
                    return false;
                }
            }
            auth.expires > Proxy::timestamp().as_secs()
        } else {
            false
        }
    }

    pub fn login(&mut self, account: &str, password: &str) -> Option<String> {
        if self.is_authenticated() {
            return Some("Authentication is already done".to_string());
        }

        let secrets = json!({
            "username": account,
            "password": password,
        });

        let resp = self
            .request(Method::POST, "/sessions")
            .header(CONTENT_TYPE, "application/json")
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

        let re = regex::Regex::new(r"^(?<key>[^=]+)=(?<value>[^;]+).*expires=(?<expires>[^;]+)")
            .unwrap();
        let cookies: HashMap<&str, (u64, &str)> =
            HashMap::from_iter(resp.headers().get_all(SET_COOKIE).iter().map(|value| {
                let result = re.captures(value.to_str().unwrap()).unwrap();
                let expires = result
                    .name("expires")
                    .unwrap()
                    .as_str()
                    .replace("-", " ")
                    .parse::<dateparser::DateTimeUtc>()
                    .unwrap()
                    .0
                    .timestamp() as u64;
                (
                    result.name("key").unwrap().as_str(),
                    (expires, result.name("value").unwrap().as_str()),
                )
            }));

        let mut auth_expires = u64::MAX;
        for key in vec![".SFCommunity", "session_APP"] {
            assert!(cookies.contains_key(key));
            let (expires, _) = cookies.get(key).unwrap();
            if expires < &auth_expires {
                auth_expires = expires.clone();
            }
        }

        let auth = AuthStatus {
            status: (true, 0, None),
            credentials: vec![AuthCredential {
                ident: None,
                token: cookies.get(".SFCommunity").unwrap().1.to_string(),
                session: cookies.get("session_APP").unwrap().1.to_string(),
                secrets: (account.to_string(), password.to_string()),
                expires: auth_expires,
            }],
        };

        self.store("auth", serde_json::to_value(auth).unwrap());

        None
    }

    pub fn logout(&mut self) -> bool {
        let authenticated = self.is_authenticated();
        if authenticated {
            let value = self.load_mut("auth").unwrap();
            let auth = serde_json::from_value::<AuthStatus>(value.to_owned()).unwrap();
            let index = auth.status.1;
            value
                .as_object_mut()
                .unwrap()
                .get_mut("credentials")
                .unwrap()
                .as_array_mut()
                .unwrap()
                .remove(index);
        }
        authenticated
    }

    pub fn profile(&self) -> Result<types::User> {
        if self.is_authenticated() {
            let resp = self.request(Method::GET, "/user").send()?;
            let status_code = resp.status();
            let data = serde_json::from_str::<Value>(&resp.text()?)?;
            if status_code == 200 {
                let data = data.as_object().unwrap().get("data").unwrap().to_owned();
                Ok(serde_json::from_value(data)?)
            } else {
                let data = data.as_object().unwrap().get("status").unwrap().to_owned();
                let status = serde_json::from_value::<types::Status>(data)?;
                bail!(status.msg.unwrap());
            }
        } else {
            bail!("authentication required");
        }
    }
}
