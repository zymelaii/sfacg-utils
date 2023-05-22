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

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct _NovelInfo {
    pub typeId: usize,
    pub signStatus: String,
    pub novelId: Option<i32>,
    pub novelName: String,
    pub authorId: Id,
    pub authorName: String,
    pub charCount: usize,
    pub isFinish: bool,
    pub viewTimes: usize,
    pub addTime: String,
    pub lastUpdateTime: String,
}

#[derive(Debug)]
pub struct NovelInfo {
    pub r#type: usize,               //<! 小说类型
    pub sign_status: String,         //<! 签约状态
    pub id: Id,                      //<! 小说 ID
    pub name: String,                //<! 小说名称
    pub author_id: Id,               //<! 作者 ID
    pub author: String,              //<! 作者名称
    pub total_chars: usize,          //<! 总字数
    pub finished: bool,              //<! 是否完结
    pub total_views: usize,          //<! 访问量
    pub add_time: Timestamp,         //<! 添加时间
    pub last_update_time: Timestamp, //<! 最近一次的更新时间
}
