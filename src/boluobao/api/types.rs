use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Status {
    pub httpCode: u32,
    pub errorCode: i32,
    pub msgType: i32,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct User {
    pub accountId: u32,
    pub countryCode: u32,
    pub avatar: String,
    pub email: String,
    pub fireCoin: i32,
    pub isAuthor: bool,
    pub nickName: String,
    pub phoneNum: String,
    pub registerDate: String,
    pub roleName: String,
    pub userName: String,
}
