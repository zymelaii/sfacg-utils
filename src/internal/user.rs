//! 用户信息

use super::*;

#[derive(Debug)]
pub struct User {
    pub id: Id,                  //<! 用户 ID
    pub uuid: String,            //<! 用户唯一标识码
    pub nickname: String,        //<! 昵称
    pub intro: String,           //<! 个人简介
    pub avatar: String,          //<! 头像 URL
    pub background: String,      //<! 背景图 URL
    pub verify_info: VerifyType, //<! 认证信息
    pub total_follows: usize,    //<! 关注数量
    pub total_fans: usize,       //<! 粉丝数量
    pub is_follow: bool,         //<! 是否被对方关注（对于操作者而言）
    pub is_followed: bool,       //<! 是否关注对方（对于操作者而言）
    pub is_blocked: bool,        //<! 是否在黑名单（对于操作者而言）
}
