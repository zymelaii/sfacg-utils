use crate::consts;

use anyhow::{bail, Error, Result};
use reqwest::{blocking::RequestBuilder, header::HeaderMap, Method};

type Timestamp = u64;
type Id = i32;

#[derive(Clone)]
pub struct Client {
    version: String,
    channel: String,
    device_id: String,
    token: Option<String>,
    session: Option<String>,
}

impl Client {
    pub fn default() -> Result<Self> {
        let version = consts::APPKEYS
            .keys()
            .next()
            .ok_or(Error::msg("no appkeys available"))?
            .to_string();

        use uuid::Uuid;
        Ok(Self {
            version: version,
            channel: "HomePage".to_string(),
            device_id: Uuid::new_v4().to_string(),
            token: None,
            session: None,
        })
    }

    fn security(&self) -> Result<String> {
        use uuid::Uuid;
        let nonce = Uuid::new_v4().to_string().to_uppercase();
        let timestamp = Self::timestamp()?.as_millis();
        let device_token = self.device_id.to_uppercase();
        let appkey = consts::APPKEYS
            .get(&self.version)
            .ok_or(Error::msg(format!(
                "appkey for version `{}` is not available",
                self.version
            )))?;

        use crypto::{digest::Digest, md5::Md5};
        let source = format!("{nonce}{timestamp}{device_token}{appkey}");
        let mut digest = Md5::new();
        digest.input_str(&source);
        let sign = digest.result_str().to_uppercase();

        Ok(format!(
            "nonce={nonce}&timestamp={timestamp}&devicetoken={device_token}&sign={sign}"
        ))
    }

    pub fn timestamp() -> Result<std::time::Duration> {
        Ok(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?)
    }

    fn user_agent(&self) -> String {
        format!(
            "boluobao/{}/{}/{}",
            self.version, self.channel, self.device_id,
        )
    }

    fn default_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let accept = "application/vnd.sfacg.api+json;version=1";
        let security = self.security()?;

        use reqwest::header::*;
        headers.insert(ACCEPT, accept.parse().unwrap());
        headers.insert(ACCEPT_CHARSET, "UTF-8".parse().unwrap());
        headers.insert(AUTHORIZATION, consts::AUTH.parse().unwrap());
        headers.insert(USER_AGENT, self.user_agent().parse().unwrap());
        headers.insert("SFSecurity", security.parse().unwrap());

        Ok(headers)
    }

    pub fn as_guest(&self) -> Self {
        let mut client = self.clone();
        client.token = None;
        client.session = None;
        client
    }

    pub fn with(&self, token: &str, session: &str) -> Self {
        let mut client = self.clone();
        client.token = Some(token.to_string());
        client.session = Some(session.to_string());
        client
    }

    pub fn request(&self, method: Method, api: &str) -> Result<RequestBuilder> {
        let client = reqwest::blocking::Client::new()
            .request(method, format!("{}{api}", consts::APIPREFIX))
            .headers(self.default_headers()?);

        use reqwest::header::*;
        Ok(if self.token.is_some() && self.session.is_some() {
            let cookies = format!(
                ".SFCommunity={}; session_APP={}",
                self.token.unwrap(),
                self.session.unwrap()
            );
            client.header(COOKIE, cookies)
        } else {
            client
        })
    }

    pub fn get(&self, api: &str) -> Result<RequestBuilder> {
        Ok(self.request(Method::GET, api)?)
    }

    pub fn post(&self, api: &str) -> Result<RequestBuilder> {
        Ok(self.request(Method::POST, api)?)
    }
}
