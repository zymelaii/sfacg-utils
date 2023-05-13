//! 1. 用户与凭证一一绑定
//! 2. Flex 可以保留多对凭证
//! 3. Flex 可以是未验证状态或已验证状态
//! 4. 只要 Flex 存在有效凭证，就必须是验证状态
//! 5.
//!

use anyhow::Result;
use directories::ProjectDirs;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use crate::{
    api::{auth::AuthStatus, types},
    internal::*,
    request::*,
    unpack_sfresp, Proxy,
};

#[derive(Deserialize, Serialize)]
struct LocalCredential {
    id: Id,                   //<! 账户 ID
    email: Option<String>,    //<! 账户邮箱
    phone: Option<String>,    //<! 账户手机号
    password: Option<String>, //<! 账户密码
    token: String,            //<! 凭据
    session: String,          //<! 会话
    expires: Timestamp,       //<! 过期时间
}

#[derive(Debug)]
pub struct Credential {
    pub id: Id,             //<! 账户 ID
    pub token: String,      //<! 凭据
    pub session: String,    //<! 会话
    pub expires: Timestamp, //<! 过期时间
}

#[derive(Debug)]
pub struct Flex {
    pub auth_id: Id,                          //<! 活跃账户
    pub auth_list: Vec<Id>,                   //<! 账户列表
    pub credentials: HashMap<Id, Credential>, //<! 凭证列表
}

impl Flex {
    /// 获取本地数据存储目录
    fn data_local_dir() -> PathBuf {
        let dirs = ProjectDirs::from("", "", "sfutils").unwrap();
        dirs.data_local_dir().to_owned()
    }

    /// 初始化本地数据存储
    fn try_init_local_storage() -> Result<()> {
        let dir = Self::data_local_dir();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        Ok(())
    }

    /// 获取本地凭证
    fn try_get_local_credentials() -> Result<HashMap<String, LocalCredential>> {
        Self::try_init_local_storage()?;
        let dir = Self::data_local_dir();
        let raw = fs::read_to_string(dir.join("auth.toml"))?;
        Ok(toml::from_str(&raw)?)
    }

    /// 存储本地凭证
    fn write_local_credentials(credentials: HashMap<String, LocalCredential>) -> Result<()> {
        Self::try_init_local_storage()?;
        let dir = Self::data_local_dir();
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(dir.join("auth.toml"))?;
        file.write_all(toml::to_string(&credentials)?.as_bytes())?;
        Ok(())
    }

    /// 验证账户
    pub fn authenticate(
        &mut self,
        account: &str,
        password: &str,
        force: bool,
        private_mode: bool,
    ) -> Result<()> {
        let mut credentials = match Self::try_get_local_credentials() {
            Ok(credentials) => credentials,
            Err(_) => HashMap::new(),
        };

        let result = credentials.iter().find(|(_, e)| {
            if e.password.is_none() {
                return false;
            }
            let email = e.email.as_ref().unwrap();
            let phone = e.phone.as_ref().unwrap();
            let pwd = e.password.as_ref().unwrap();
            pwd == password && (phone == account || email == account)
        });

        if let Some((k, e)) = result {
            if !force && self.credentials.contains_key(&e.id) {
                return Ok(());
            }
            credentials.remove(&k.to_owned());
        }

        let mut proxy = Proxy::default();
        proxy.login(account, password)?;
        let auth =
            serde_json::from_value::<AuthStatus>(proxy.load("auth").unwrap().to_owned()).unwrap();
        let credential = auth.credentials.get(auth.status.1).unwrap();
        let resp = proxy
            .request_with(Method::GET, "/user", &credential.token, &credential.session)
            .send()?;
        let data = || -> Result<types::UserPrivate> {
            unpack_sfresp!(resp);
        }()?;

        let credential = LocalCredential {
            id: data.accountId,
            email: if private_mode { None } else { Some(data.email) },
            phone: if private_mode {
                None
            } else {
                Some(data.phoneNum)
            },
            password: if private_mode {
                None
            } else {
                Some(password.to_string())
            },
            token: credential.token.to_owned(),
            session: credential.session.to_owned(),
            expires: credential.expires as i64,
        };

        if self.auth_id == 0 {
            self.auth_id = credential.id;
        }
        if !self.credentials.contains_key(&credential.id) {
            self.auth_list.push(credential.id);
        }
        self.credentials.insert(
            credential.id,
            Credential {
                id: credential.id,
                token: credential.token.clone(),
                session: credential.session.clone(),
                expires: credential.expires.clone(),
            },
        );

        credentials.insert(data.nickName, credential);

        Self::write_local_credentials(credentials)?;
        Ok(())
    }

    /// 从本地凭证构造
    pub fn from_local() -> Result<Self> {
        Self::try_init_local_storage()?;
        if let Ok(credentials) = Self::try_get_local_credentials() {
            let proxy = Proxy::default();
            let timestamp = Proxy::timestamp().as_secs() as i64;
            let auth_list: Vec<Credential> = credentials
                .into_iter()
                .filter(|(_, v)| v.expires > timestamp)
                .filter_map(|(_, e)| {
                    match proxy
                        .request_with(Method::GET, "/user", &e.token, &e.session)
                        .send()
                    {
                        Ok(resp) => match resp.status() {
                            StatusCode::OK => Some(Credential {
                                id: e.id,
                                token: e.token.to_owned(),
                                session: e.session.to_owned(),
                                expires: e.expires,
                            }),
                            _ => None,
                        },
                        Err(_) => None,
                    }
                })
                .collect();
            Ok(Flex {
                auth_id: if auth_list.is_empty() {
                    0
                } else {
                    auth_list.get(0).unwrap().id
                },
                auth_list: auth_list.iter().map(|e| e.id).collect(),
                credentials: auth_list.into_iter().map(|e| (e.id, e)).collect(),
            })
        } else {
            Ok(Flex {
                auth_id: 0,
                auth_list: vec![],
                credentials: HashMap::new(),
            })
        }
    }
}
