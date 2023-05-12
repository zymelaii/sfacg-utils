use super::types;
use crate::{consts, request::*, unpack_sfresp, Proxy};

use anyhow::Result;

impl Proxy {
    pub fn novel_info(&self, novel_id: i32) -> Result<types::Novel> {
        unpack_sfresp!(self
            .request(Method::GET, &format!("/novels/{novel_id}"))
            .query(&[("expand", consts::FULLEXPAND["novels"])])
            .send()?);
    }

    pub fn catalogue_of(&self, novel_id: i32) -> Result<types::Catalogue> {
        unpack_sfresp!(self
            .request(Method::GET, &format!("/novels/{novel_id}/dirs"))
            .query(&[("expand", consts::FULLEXPAND["novels/dirs"])])
            .send()?);
    }
}
