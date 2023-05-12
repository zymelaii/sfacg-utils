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

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SystemTag {
    sysTagId: i32,
    tagName: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RankingList {
    dateRange: i32,
    desc: String,
    r#type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ChapterRef {
    addTime: String,
    chapId: i32,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NovelExpand {
    pub intro: Option<String>,
    pub signLevel: Option<String>,
    pub chapterCount: Option<usize>,
    pub auditCover: Option<String>,
    pub bigBgBanner: Option<String>,
    pub bigNovelCover: Option<String>,
    pub customTag: Option<Vec<String>>,
    pub firstChapter: Option<ChapterRef>,
    pub lastChapter: Option<ChapterRef>,
    pub discount: Option<f32>,
    pub discountExpireDate: Option<String>,
    pub fav: Option<usize>,
    pub essayTag: Option<String>,
    pub unauditedCustomtag: Option<Vec<String>>,
    pub homeFlag: Option<Vec<String>>,
    pub isBanch: Option<bool>,
    pub latestCommentDate: Option<String>,
    pub pointCount: Option<usize>,
    pub preOrderInfo: Option<String>,
    pub rankinglist: Option<RankingList>,
    pub sysTags: Option<Vec<SystemTag>>,
    pub tags: Option<Vec<String>>,
    pub topic: Option<String>,
    pub ticket: Option<usize>,
    pub typeName: Option<String>,
    pub originTotalNeedFireMoney: Option<usize>,
    pub totalNeedFireMoney: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Novel {
    pub addTime: String,
    pub allowDown: bool,
    pub authorId: i32,
    pub authorName: String,
    pub bgBanner: String,
    pub categoryId: i32,
    pub charCount: usize,
    pub isFinish: bool,
    pub isSensitive: bool,
    pub lastUpdateTime: String,
    pub markCount: usize,
    pub novelName: String,
    pub point: f32,
    pub signStatus: String,
    pub typeId: usize,
    pub viewTimes: usize,
    pub expand: Option<NovelExpand>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Chapter {
    pub novelId: i32,
    pub volumeId: i32,
    pub chapId: i32,
    pub title: String,
    pub isVip: bool,
    pub chapOrder: i32,
    pub rowNum: usize,
    pub charCount: usize,
    pub AddTime: String,
    pub updateTime: Option<String>,
    pub isRubbish: bool,
    pub auditStatus: i32,
    pub chapterOriginFireMoney: usize,
    pub content: Option<String>,
    pub needFireMoney: usize,
    pub originNeedFireMoney: usize,
    pub canUnlockWithAd: bool,
    pub ntitle: String,
    pub sno: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Volume {
    pub title: String,
    pub volumeId: i32,
    pub chapterList: Vec<Chapter>,
    pub sno: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Catalogue {
    pub lastUpdateTime: String,
    pub novelId: i32,
    pub volumeList: Vec<Volume>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NovelRef {
    pub allowDown: bool,
    pub authorId: i32,
    pub authorName: String,
    pub bgBanner: String,
    pub categoryId: i32,
    pub charCount: usize,
    pub isFinish: bool,
    pub isSensitive: bool,
    pub isSticky: bool,
    pub lastUpdateTime: String,
    pub markCount: usize,
    pub markDateTime: String,
    pub novelCover: String,
    pub novelId: i32,
    pub novelName: String,
    pub point: f32,
    pub signStatus: String,
    pub stickyDateTime: Option<String>,
    pub typeId: i32,
    pub viewTimes: usize,
    pub expand: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PocketExpand {
    pub novels: Option<Vec<NovelRef>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Pocket {
    pub accountId: i32,
    pub canModify: bool,
    pub createTime: String,
    pub isFull: bool,
    pub name: String,
    pub pocketId: i32,
    pub typeId: i32,
    pub expand: Option<PocketExpand>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NovelRecord {
    pub allowDown: bool,
    pub authorId: i32,
    pub authorName: String,
    pub addTime: String,
    pub bgBanner: String,
    pub categoryId: i32,
    pub charCount: usize,
    pub isFinish: bool,
    pub isSensitive: bool,
    pub lastUpdateTime: String,
    pub markCount: usize,
    pub novelCover: String,
    pub novelId: i32,
    pub novelName: String,
    pub point: f32,
    pub signStatus: String,
    pub typeId: i32,
    pub viewTimes: usize,
    pub weight: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SearchResult {
    pub albums: Vec<serde_json::Value>,
    pub comics: Vec<serde_json::Value>,
    pub novels: Vec<NovelRecord>,
}
