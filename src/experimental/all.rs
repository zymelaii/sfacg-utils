use std::{cmp::min, collections::HashMap, ops::Index, path::PathBuf, str::FromStr};

use super::types;
use crate::consts;

use anyhow::{bail, Error, Result};
use reqwest::{blocking::RequestBuilder, header::HeaderMap, Method, StatusCode};
use serde::{Deserialize, Serialize};

pub type Timestamp = u64;
pub type Id = i32;

#[derive(Debug, Clone)]
pub struct Client {
    version: String,
    channel: String,
    device_id: String,
    token: Option<String>,
    session: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PrivateCredential {
    owner_id: Id,          //<! 用户 ID
    owner: Option<String>, //<! 用户名
    key: Option<String>,   //<! 密钥
    email: Option<String>, //<! 关联邮箱
    phone: Option<String>, //<! 关联手机号
    session: String,       //<! 会话 ID
    token: String,         //<! 凭据
    expires: Timestamp,    //<! 会话有效期限
    is_dirty: bool,        //<! 是否失效
    is_broken: bool,       //<! 是否损坏
}

#[derive(Debug)]
pub struct Host {
    client: Client,
    major_auth: Id,
    active_list: Vec<Id>,
    credentials: Vec<PrivateCredential>,
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

    pub fn with(&self, token: &str, session: &str) -> Self {
        let mut client = self.clone();
        client.token = Some(token.to_string());
        client.session = Some(session.to_string());
        client
    }

    pub fn as_guest(&self) -> Self {
        let mut client = self.clone();
        client.token = None;
        client.session = None;
        client
    }
}

impl Client {
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
}

impl Client {
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

    pub fn request(&self, method: Method, api: &str) -> Result<RequestBuilder> {
        let client = reqwest::blocking::Client::new()
            .request(method, format!("{}{api}", consts::APIPREFIX))
            .headers(self.default_headers()?);

        use reqwest::header::*;
        Ok(if self.token.is_some() && self.session.is_some() {
            let cookies = format!(
                ".SFCommunity={}; session_APP={}",
                self.token.as_ref().unwrap(),
                self.session.as_ref().unwrap()
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

impl Host {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::default()?,
            major_auth: 0,
            active_list: vec![],
            credentials: vec![],
        })
    }
}

impl Host {
    /// 获取登录状态
    fn _auth_info(&self, token: &str, session: &str) -> Result<types::AuthInfo> {
        use serde_json::Value;

        let resp = self.client.with(token, session).get("/user")?.send()?;

        let status_code = resp.status();
        let data = resp.text()?.parse::<Value>()?;

        if status_code != StatusCode::OK {
            let msg = data
                .get("status")
                .ok_or(Error::msg("bad-formed api request"))?
                .get("msg")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            bail!(msg);
        }

        let data = data.get("data").unwrap().to_owned();
        let info = serde_json::from_value::<types::_AuthInfo>(data)?;

        let timestamp = format!("{}Z", info.registerDate)
            .parse::<dateparser::DateTimeUtc>()?
            .0
            .timestamp() as Timestamp;

        Ok(types::AuthInfo {
            id: info.accountId,
            device_id: self.client.device_id.to_owned(),
            area_code: info.countryCode,
            nickname: info.nickName,
            phone: if info.phoneNum.is_empty() {
                None
            } else {
                Some(info.phoneNum)
            },
            email: if info.email.is_empty() {
                None
            } else {
                Some(info.email)
            },
            registration_time: timestamp,
        })
    }

    /// 验证身份
    fn _authenticate(&mut self, account: &str, password: &str) -> Result<PrivateCredential> {
        use reqwest::header::*;
        use serde_json::{json, Value};

        let params = json!({
            "username": account,
            "password": password,
        });

        let resp = self
            .client
            .as_guest()
            .post("/sessions")?
            .header(CONTENT_TYPE, "application/json")
            .body(params.to_string())
            .send()?;

        let stautus_code = resp.status();
        let headers = resp.headers().to_owned();
        let data = resp
            .text()?
            .parse::<Value>()?
            .as_object()
            .unwrap()
            .to_owned();

        if stautus_code != StatusCode::OK {
            let msg = data
                .get("status")
                .ok_or(Error::msg("bad-formed api request"))?
                .get("msg")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            bail!(msg);
        };

        let mut token = String::default();
        let mut session = String::default();
        let mut expires = Timestamp::MAX;

        let re = regex::Regex::new(r"^(?<key>[^=]+)=(?<value>[^;]+).*expires=(?<expires>[^;]+)")
            .unwrap();

        headers.get_all(SET_COOKIE).iter().for_each(|e| {
            let result = re.captures(e.to_str().unwrap()).unwrap();
            let key = result.name("key").unwrap().as_str();
            let value = result.name("value").unwrap().as_str();
            let _expires = result
                .name("expires")
                .unwrap()
                .as_str()
                .replace("-", " ")
                .parse::<dateparser::DateTimeUtc>()
                .unwrap()
                .0
                .timestamp() as Timestamp;
            if key == ".SFCommunity" {
                token = value.to_owned();
                expires = min(expires, _expires);
            } else if key == "session_APP" {
                session = value.to_owned();
                expires = min(expires, _expires);
            }
        });

        let auth = self._auth_info(&token, &session)?;

        Ok(PrivateCredential {
            owner_id: auth.id,
            owner: Some(auth.nickname),
            key: Some(password.to_owned()),
            email: auth.email,
            phone: auth.phone,
            session: session,
            token: token,
            expires: expires,
            is_dirty: false,
            is_broken: false,
        })
    }

    fn _novel_info(&self, novel_id: Id) -> Result<types::NovelInfo> {
        use serde_json::Value;

        let resp = self
            .client
            .as_guest()
            .get(&format!("/novels/{novel_id}"))?
            .send()?;

        let status_code = resp.status();
        let data = resp.text()?.parse::<Value>()?;

        if status_code != StatusCode::OK {
            let msg = data
                .get("status")
                .ok_or(Error::msg("bad-formed api request"))?
                .get("msg")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            bail!(msg);
        }

        let data = data.get("data").unwrap().to_owned();
        let info = serde_json::from_value::<types::_NovelInfo>(data)?;

        let add_time = format!("{}Z", info.addTime)
            .parse::<dateparser::DateTimeUtc>()?
            .0
            .timestamp() as Timestamp;

        let last_update_time = format!("{}Z", info.lastUpdateTime)
            .parse::<dateparser::DateTimeUtc>()?
            .0
            .timestamp() as Timestamp;

        Ok(types::NovelInfo {
            r#type: info.typeId,
            sign_status: info.signStatus,
            id: novel_id,
            name: info.novelName,
            author_id: info.authorId,
            author: info.authorName,
            total_chars: info.charCount,
            finished: info.isFinish,
            total_views: info.viewTimes,
            add_time: add_time,
            last_update_time: last_update_time,
        })
    }
}

impl Host {
    pub fn auth_info(&self) -> Result<types::AuthInfo> {
        let result = self
            .credentials
            .iter()
            .find(|e| e.owner_id == self.major_auth)
            .ok_or(Error::msg("unknown auth"))?;
        Ok(self._auth_info(&result.token, &result.session)?)
    }

    pub fn authenticate(&mut self, account: &str, password: &str) -> Result<Id> {
        let credential = self._authenticate(account, password)?;
        let id = credential.owner_id;
        if let Some(e) = self.credentials.iter_mut().find(|e| e.owner_id == id) {
            *e = credential;
        } else {
            self.credentials.push(credential);
        }
        Ok(id)
    }

    pub fn novel_info(&self, novel_id: Id) -> Result<types::NovelInfo> {
        Ok(self._novel_info(novel_id)?)
    }

    pub fn login(&mut self, id: Id, as_major: bool) -> Result<()> {
        if !self.active_list.contains(&id) {
            if let Some(credential) = self.credentials.iter().find(|e| e.owner_id == id) {
                let expired = credential.expires <= Client::timestamp()?.as_secs();
                if expired {
                    bail!("credential is expired");
                } else if credential.is_dirty {
                    bail!("credential is dirty");
                } else if credential.is_broken {
                    bail!("credential is broken");
                } else {
                    self.active_list.push(id);
                }
            } else {
                bail!("authentication required");
            }
        }
        if self.major_auth == 0 || as_major {
            self.major_auth = id;
        }
        Ok(())
    }

    pub fn logout(&mut self, id: Id) -> Result<Id> {
        if !self.active_list.contains(&id) {
            bail!("logged-in user expected");
        }
        let (index, _) = self
            .active_list
            .iter()
            .enumerate()
            .find(|(_, e)| **e == id)
            .unwrap();
        self.active_list.remove(index);
        if id == self.major_auth {
            self.major_auth = if self.active_list.is_empty() {
                0
            } else {
                self.active_list.index(0).to_owned()
            }
        }
        Ok(self.major_auth)
    }
}

impl Host {
    pub fn from_local(path: &str) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        type ResultType = HashMap<String, PrivateCredential>;
        let data = toml::from_str::<ResultType>(&String::from_utf8(bytes)?)?;
        let credentials: Vec<PrivateCredential> = data.into_values().collect();
        let mut resp = Self::new()?;
        let timestamp = Client::timestamp()?.as_secs();
        resp.credentials = credentials
            .into_iter()
            .filter(|e| !e.is_broken && !e.is_dirty && e.expires > timestamp)
            .collect();
        if resp.credentials.len() > 0 {
            resp.major_auth = resp.credentials.get(0).unwrap().owner_id;
        }
        Ok(resp)
    }

    pub fn dump_to_local(&self, path: &str) -> Result<()> {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(PathBuf::from_str(path)?)?;
        let data = HashMap::<String, &PrivateCredential>::from_iter(
            self.credentials
                .iter()
                .map(|e| (e.owner.as_ref().unwrap().to_owned(), e)),
        );
        file.write_all(toml::to_string(&data)?.as_bytes())?;
        Ok(())
    }
}
