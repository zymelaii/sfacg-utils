pub mod auth;
pub mod chapter;
pub mod favorites;
pub mod novel;
pub mod search;
pub mod types;

#[macro_export]
macro_rules! unpack_sfresp {
    ($resp:expr) => {
        let status_code = $resp.status();
        let data = serde_json::from_str::<serde_json::Value>(&$resp.text()?)?;
        if status_code == 200 {
            let data = data.as_object().unwrap().get("data").unwrap().to_owned();
            return Ok(serde_json::from_value(data)?);
        } else {
            let data = data.as_object().unwrap().get("status").unwrap().to_owned();
            let status = serde_json::from_value::<types::Status>(data)?;
            anyhow::bail!(status.msg.unwrap());
        }
    };
}
