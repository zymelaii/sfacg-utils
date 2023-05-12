use super::types;
use crate::{consts, request::*, unpack_sfresp, Proxy};

use anyhow::Result;

impl Proxy {
    pub fn favoirtes(&self) -> Result<Vec<types::Pocket>> {
        unpack_sfresp!(self
            .request(Method::GET, &format!("/user/Pockets"))
            .query(&[("expand", consts::FULLEXPAND["user/pockets"])])
            .send()?);
    }
}
