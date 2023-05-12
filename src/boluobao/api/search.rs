use super::types;
use crate::{request::*, unpack_sfresp, Proxy};

use anyhow::Result;

impl Proxy {
    pub fn search(&self, keyword: &str, page: usize, size: usize) -> Result<types::SearchResult> {
        unpack_sfresp!(self
            .request(Method::GET, &format!("/search/novels/result"))
            .query(&[
                ("q", keyword),
                ("page", &page.to_string()),
                ("size", &size.to_string())
            ])
            .send()?);
    }
}
