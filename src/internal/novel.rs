////! 小说信息

use super::*;

#[derive(Debug)]
pub struct Chapter {
    pub novel_id: Id,             //<! 小说 ID
    pub volume_id: Id,            //<! 卷 ID
    pub id: Id,                   //<! 章节 ID
    pub title: String,            //<! 标题
    pub order: usize,             //<! 章节序号
    pub total_chars: usize,       //<! 总字数
    pub creation_time: Timestamp, //<! 创建时间
    pub update_time: Timestamp,   //<! 最后更新时间
    pub is_free: bool,            //<! 是否是免费章节
    pub price: usize,             //<! 现价
    pub origin_price: usize,      //<! 原价
    pub content: Option<String>,  //<! 章节内容
}

pub struct Volume {
    pub novel_id: Id,           //<! 小说 ID
    pub id: Id,                 //<! 卷 ID
    pub title: String,          //<! 卷名
    pub order: usize,           //<! 卷序号
    pub chapters: Vec<Chapter>, //<! 章节列表
}

/// TODO: 添加价格信息
///
/// TODO: 添加小说标签信息

#[derive(Debug)]
pub struct Novel {
    pub name: String,                //<! 书名
    pub id: Id,                      //<! 小说 ID
    pub author: String,              //<! 作者
    pub author_id: Id,               //<! 作者 ID
    pub r#type: String,              //<! 小说类型
    pub intro: String,               //<! 简介
    pub sign_status: String,         //<! 签约状态
    pub sign_level: String,          //<! 签约等级
    pub total_chars: usize,          //<! 总字数
    pub total_chapters: usize,       //<! 总章节数
    pub total_views: usize,          //<! 访问次数
    pub total_likes: usize,          //<! 总点赞量
    pub total_favorites: usize,      //<! 总收藏数
    pub total_tickets: usize,        //<! 当月票数
    pub cover: String,               //<! 封面 URL
    pub banner: String,              //<! 背景横幅 URL
    pub is_finished: bool,           //<! 是否完结
    pub last_update_time: Timestamp, //<! 最后一次更新时间
    pub creation_time: Timestamp,    //<! 创建时间
}
