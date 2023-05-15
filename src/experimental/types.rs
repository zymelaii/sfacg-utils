use serde::*;

use super::all::{Id, Timestamp};

//pub mod origin {
//    use serde::{Deserialize, Serialize};
//    #[derive(Debug, Serialize, Deserialize)]
//    pub struct UserPrivate {
//        #[serde(alias = "accountId")]
//        pub id: i32,
//        pub nickName: String,
//        pub userName: String,
//        pub countryCode: u32,
//        pub avatar: String,
//        pub email: String,
//        pub fireCoin: i32,
//        pub isAuthor: bool,
//        pub phoneNum: String,
//        pub registerDate: String,
//        pub roleName: String,
//    }
//}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct _AuthInfo {
    pub accountId: i32,
    pub nickName: String,
    pub countryCode: u32,
    pub avatar: String,
    pub email: String,
    pub phoneNum: String,
    pub registerDate: String,
}

#[derive(Debug)]
pub struct AuthInfo {
    pub id: Id,                       //<! 账户 ID
    pub device_id: String,            //<! 当前设备 ID
    pub area_code: u32,               //<! 区号
    pub nickname: String,             //<! 账户昵称
    pub phone: Option<String>,        //<! 关联手机号
    pub email: Option<String>,        //<! 关联邮箱
    pub registration_time: Timestamp, //<! 注册时间
}
