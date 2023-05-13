use anyhow::Result;

use crate::boluobao::api::types;
use crate::internal::*;

impl types::Pocket {
    #[inline]
    pub fn parse(self) -> Result<Favoirtes> {
        let r#type = Type::from(self.typeId);
        let expand = self.expand.unwrap();
        let refs: Vec<Id> = match r#type {
            Type::Comic => expand.comics.unwrap().iter().map(|e| e.comicId).collect(),
            Type::Novel => expand.novels.unwrap().iter().map(|e| e.novelId).collect(),
            Type::Album => expand.albums.unwrap().iter().map(|e| e.albumId).collect(),
        };
        Ok(Favoirtes {
            id: self.pocketId,
            owner_id: self.accountId,
            name: self.name,
            r#type: r#type,
            creation_time: to_timestamp(&self.createTime)?,
            modifiable: self.canModify,
            is_full: self.isFull,
            refs: refs,
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
        let vip_info = expand.vipInfo.unwrap();
        let vip = VipInfo {
            point: vip_info.point,
            level: vip_info.level,
            next_level: vip_info.nextLevel,
            point_of_next_level: vip_info.nextLevelPoint,
            discount: vip_info.discount as usize,
            next_discount: vip_info.nextDiscount as usize,
            level_of_next_discount: vip_info.nextDiscountLevel,
            point_of_next_discount: vip_info.nextDiscountLevelPoint as usize,
        };
        Ok(UserPrivate {
            id: self.accountId,
            area_code: self.countryCode,
            phone: self.phoneNum,
            email: self.email,
            is_author: self.isAuthor,
            vip: vip,
            registration_time: to_timestamp(&self.registerDate)?,
            banlance: 0, //<! FIXME
            vouchers: 0, //<! FIXME
            tokens: expand.welfareCoin.unwrap() as usize,
        })
    }
}

impl types::Novel {
    #[inline]
    pub fn parse(self) -> Result<Novel> {
        let expand = self.expand.unwrap();
        Ok(Novel {
            name: self.novelName,
            id: self.novelId.unwrap(),
            author: self.authorName,
            author_id: self.authorId,
            r#type: expand.typeName.unwrap(),
            intro: expand.intro.unwrap(),
            sign_status: self.signStatus,
            sign_level: expand.signLevel.unwrap(),
            total_chars: self.charCount,
            total_chapters: expand.chapterCount.unwrap(),
            total_views: self.viewTimes,
            total_likes: expand.fav.unwrap(),
            total_favorites: self.markCount,
            total_tickets: expand.ticket.unwrap(),
            cover: expand.bigNovelCover.unwrap(),
            banner: expand.bigBgBanner.unwrap(),
            is_finished: self.isFinish,
            last_update_time: to_timestamp(&self.lastUpdateTime)?,
            creation_time: to_timestamp(&self.addTime)?,
        })
    }
}

impl types::Chapter {
    #[inline]
    pub fn parse(self) -> Result<Chapter> {
        let creation = to_timestamp(&self.AddTime)?;
        Ok(Chapter {
            novel_id: self.novelId,
            volume_id: self.volumeId,
            id: self.chapId,
            title: self.title,
            order: self.chapOrder as usize,
            total_chars: self.charCount,
            creation_time: creation,
            update_time: if self.updateTime.is_some() {
                to_timestamp(&self.updateTime.unwrap())?
            } else {
                creation
            },
            is_free: !self.isVip,
            price: self.needFireMoney,
            origin_price: self.chapterOriginFireMoney,
            content: self.content,
        })
    }
}
