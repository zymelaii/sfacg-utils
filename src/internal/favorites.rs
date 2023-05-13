//! 收藏夹

use super::*;

pub struct Favoirtes {
    pub id: Id,                   //<! 收藏夹 ID
    pub owner_id: Id,             //<! 所有者 ID
    pub name: String,             //<! 收藏夹命名
    pub r#type: Type,             //<! 收藏夹内容物的类型
    pub creation_time: Timestamp, //<! 创建时间
    pub modifiable: bool,         //<! 是否可修改（对于操作者而言）
    pub is_full: bool,            //<! 容量是否达到上限
}
