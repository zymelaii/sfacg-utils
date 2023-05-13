//! 用户个人信息

use super::*;

/// TODO: 增加是否允许更改昵称的信息
///
/// # 相关结构
/// { changeNickNameInfo: { canChange: bool, nextChangeNeedDays: usize } }
///
/// # 改名规则
/// 1. VIP 1 以上每年可以修改一次，每次扣除 750 余额
/// 2. VIP 5 及以上免费，但仍受时间限制
/// 3. 使用改名卡修改昵称不受时间限制
///
/// TODO: 增加余额信息（火卷）

#[derive(Debug)]
pub struct VipInfo {
    pub point: usize,                  //<! 当前点数
    pub level: usize,                  //<! 当前等级
    pub next_level: usize,             //<! 下一个等级
    pub point_of_next_level: usize,    //<! 下一个等级所需的总点数
    pub discount: usize,               //<! 当前等级的折扣
    pub next_discount: usize,          //<! 下一个折扣点的折扣
    pub level_of_next_discount: usize, //<! 下一个折扣点所在的等级
    pub point_of_next_discount: usize, //<! 下一个折扣点所需的总点数
}

#[derive(Debug)]
pub struct UserPrivate {
    pub id: Id,                       //<! 用户 ID
    pub area_code: u32,               //<! 手机区号
    pub phone: String,                //<! 绑定的手机号码
    pub email: String,                //<! 绑定的电子邮箱
    pub is_author: bool,              //<! 是否是作者
    pub vip: VipInfo,                 //<! VIP 信息
    pub registration_time: Timestamp, //<! 注册时间
    pub banlance: usize,              //<! 账户余额
    pub vouchers: usize,              //<! 代金卷数量（用于替代余额）
    pub tokens: usize,                //<! 代币数量（用于兑换福利物品）
}
