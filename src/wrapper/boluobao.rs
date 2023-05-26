use anyhow::bail;
use serde::{de::DeserializeOwned, Deserialize};
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct ResponseStatus {
    #[serde(alias = "errorCode")]
    pub error_code: i32,
    #[serde(alias = "httpCode")]
    pub http_code: i32,
    pub msg: Option<String>,
    #[serde(alias = "msgType")]
    pub msg_type: i32,
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    #[serde(alias = "Message")]
    pub message: Option<String>,
    #[serde(alias = "MessageDetail")]
    pub detail: Option<String>,
    pub status: Option<ResponseStatus>,
    data: Option<T>,
}

#[derive(Deserialize, Debug)]
pub struct NoneType {}

impl<T> FromStr for Response<T>
where
    T: DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(&s)?)
    }
}

impl<T> Response<T> {
    pub fn data(self) -> anyhow::Result<T> {
        if self.status.is_none() {
            bail!("bad-formed api request: {}", self.detail.unwrap());
        } else if self.status.as_ref().unwrap().http_code != 200 {
            bail!(self.status.unwrap().msg.unwrap());
        }
        Ok(self.data.unwrap())
    }
}
