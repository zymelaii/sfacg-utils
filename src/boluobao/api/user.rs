use super::types;
use crate::{consts, request::*, unpack_sfresp, Proxy};

use anyhow::Result;

impl Proxy {
    pub fn batch_user_info(&self, user_ids: &Vec<i32>) -> Result<Vec<types::User>> {
        let uids = user_ids
            .iter()
            .map(i32::to_string)
            .collect::<Vec<String>>()
            .join(",");
        unpack_sfresp!(self
            .request(Method::GET, &format!("/users"))
            .query(&[("expand", consts::FULLEXPAND["user"]), ("uids", &uids)])
            .send()?);
    }

    pub fn user_info(&self, user_id: i32) -> Result<types::User> {
        unpack_sfresp!(self
            .request(Method::GET, &format!("/users/{user_id}"))
            .query(&[("expand", consts::FULLEXPAND["user"])])
            .send()?);
    }

    pub fn sign_info(&self) -> Result<Vec<types::SignInfo>> {
        unpack_sfresp!(self
            .request(Method::GET, &format!("/user/signInfo"))
            .send()?);
    }
}
