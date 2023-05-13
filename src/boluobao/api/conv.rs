use anyhow::Result;

use super::types;
use crate::internal::*;

impl types::Pocket {
    #[inline]
    pub fn parse(self) -> Result<Favoirtes> {
        Ok(Favoirtes {
            id: self.pocketId,
            owner_id: self.accountId,
            name: self.name,
            r#type: Type::from(self.typeId),
            creation_time: to_timestamp(&self.createTime)?,
            modifiable: self.canModify,
            is_full: self.isFull,
        })
    }
}

impl types::User {
    #[inline]
    pub fn parse(self) -> Result<User> {
        let expand = self.expand.unwrap();
        Ok(User {
            id: self.accountId,
            uuid: self.userName,
            nickname: self.nickName,
            intro: expand.introduction.unwrap(),
            avatar: expand.bigAvatar.unwrap(),
            background: expand.backgroundPic.unwrap(),
            verify_info: VerifyType::from(expand.verifyType.unwrap(), expand.verifyInfo.unwrap()),
            total_follows: expand.followNum.unwrap(),
            total_fans: expand.fansNum.unwrap(),
            is_follow: expand.youfollow.unwrap(),
            is_followed: expand.followyou.unwrap(),
            is_blocked: expand.youblock.unwrap(),
        })
    }
}

impl types::UserPrivate {
    #[inline]
    pub fn parse(self) -> Result<UserPrivate> {
        let expand = self.expand.unwrap();
        Ok(UserPrivate {
            id: self.accountId,
            area_code: self.countryCode,
            phone: self.phoneNum,
            email: self.email,
            is_author: self.isAuthor,
            registration_time: to_timestamp(&self.registerDate)?,
            banlance: 0, //<! FIXME
            vouchers: 0, //<! FIXME
            tokens: expand.welfareCoin.unwrap() as usize,
        })
    }
}
