use serde::*;

use super::all::{Id, Timestamp};

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    let result = format!("{}Z", s).parse::<dateparser::DateTimeUtc>();
    Ok(result.unwrap().0.timestamp() as Timestamp)
}

fn deserialize_non_empty_str<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    Ok(if s.is_empty() { None } else { Some(s) })
}

#[derive(Debug, Deserialize)]
pub struct AuthInfo {
    #[serde(alias = "accountId")]
    pub id: Id,                         //<! 账户 ID
    #[serde(default)]
    pub device_id: String,              //<! 当前设备 ID
    #[serde(alias = "countryCode")]
    pub area_code: u32,                 //<! 区号
    #[serde(alias = "nickName")]
    pub nickname: String,               //<! 账户昵称
    #[serde(deserialize_with = "deserialize_non_empty_str")]
    pub email: Option<String>,          //<! 关联邮箱
    #[serde(alias = "phoneNum", deserialize_with = "deserialize_non_empty_str")]
    pub phone: Option<String>,          //<! 关联手机号
    #[serde(alias = "registerDate", deserialize_with = "deserialize_timestamp")]
    pub registration_time: Timestamp,   //<! 注册时间
}

#[derive(Debug, Deserialize)]
pub struct NovelInfo {
    #[serde(alias = "typeId")]
    pub r#type: usize,                  //<! 小说类型
    #[serde(alias = "signStatus")]
    pub sign_status: String,            //<! 签约状态
    #[serde(alias = "novelId")]
    pub id: Id,                         //<! 小说 ID
    #[serde(alias = "novelName")]
    pub name: String,                   //<! 小说名称
    #[serde(alias = "authorId")]
    pub author_id: Id,                  //<! 作者 ID
    #[serde(alias = "authorName")]
    pub author: String,                 //<! 作者名称
    #[serde(default)]
    pub brief: String,                  //<! 小说简介
    #[serde(default)]
    pub cover: String,                  //<! 小说封面
    #[serde(alias = "charCount")]
    pub total_chars: usize,             //<! 总字数
    #[serde(default)]
    pub total_chapters: usize,          //<! 总章节数
    #[serde(alias = "viewTimes")]
    pub total_views: usize,             //<! 访问量
    #[serde(alias = "isFinish")]
    pub finished: bool,                 //<! 是否完结
    #[serde(alias = "addTime", deserialize_with = "deserialize_timestamp")]
    pub add_time: Timestamp,            //<! 添加时间
    #[serde(alias = "lastUpdateTime", deserialize_with = "deserialize_timestamp")]
    pub last_update_time: Timestamp,    //<! 最近一次的更新时间
}
