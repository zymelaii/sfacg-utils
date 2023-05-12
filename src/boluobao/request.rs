use super::{api::auth::AuthStatus, *};

use reqwest::{
    blocking::{Client, RequestBuilder},
    header::HeaderMap,
};
pub use reqwest::{
    header::{ACCEPT, ACCEPT_CHARSET, AUTHORIZATION, CONTENT_TYPE, COOKIE, SET_COOKIE, USER_AGENT},
    Method,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn timestamp() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

impl Proxy {
    #[inline]
    pub fn timestamp() -> Duration {
        timestamp()
    }

    fn user_agent(&self) -> String {
        format!(
            "boluobao/{}/{}/{}",
            self.get_app_version(),
            self.get_channel(),
            self.get_device_token(),
        )
    }

    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let accept = "application/vnd.sfacg.api+json;version=1";
        let security = get_sfsecurity(&self.get_app_version(), &self.get_device_token());

        headers.insert(ACCEPT, accept.parse().unwrap());
        headers.insert(ACCEPT_CHARSET, "UTF-8".parse().unwrap());
        headers.insert(AUTHORIZATION, consts::AUTH.parse().unwrap());
        headers.insert(USER_AGENT, self.user_agent().parse().unwrap());
        headers.insert("SFSecurity", security.parse().unwrap());

        headers
    }

    pub fn request(&self, method: Method, api: &str) -> RequestBuilder {
        let prefix = consts::APIPREFIX;
        let client = Client::new();
        let client = client
            .request(method, format!("{prefix}/{api}"))
            .headers(self.default_headers());
        if self.is_authenticated() {
            let value = self.load("auth").unwrap().to_owned();
            let auth = serde_json::from_value::<AuthStatus>(value).unwrap();
            let index = auth.status.1;
            let credential = &auth.credentials[index];
            client.header(
                COOKIE,
                format!(
                    ".SFCommunity={}; session_APP={}",
                    credential.token, credential.session
                ),
            )
        } else {
            client
        }
    }
}
