# 获取用户的关注列表

> /users/{userId}/follows

|  参数  |       含义       |         可选值         |
| :----: | :--------------: | :--------------------: |
| expand | 需要展开的信息项 | 同用户信息，默认不展开 |
|  page  |                  |           0            |
|  size  |                  |           20           |

# 获取用户的粉丝列表

> /users/{userId}/fans

|  参数  |       含义       |         可选值         |
| :----: | :--------------: | :--------------------: |
| expand | 需要展开的信息项 | 同用户信息，默认不展开 |
|  page  |                  |           0            |
|  size  |                  |           20           |

# 当前用户的签到记录

> /user/signInfo

# 小说的粉丝榜

> /novels/{novelId}/fans

# 当前用户的作者信息

> /user/authorInfo

# 当前用户的消费记录

> /user/consumes

|   参数   |        含义         |                可选值                 |
| :------: | :-----------------: | :-----------------------------------: |
|   type   |   消费目标的类型    |     见下，按逗号分隔，默认为全部      |
| entityId | 消费目标所在项的 ID | 如可以用 novelId 指定小说，默认为全部 |
|   page   |                     |                   0                   |
|   size   |                     |                  15                   |

**type**

- chaps: 小说章节
- comic: 漫画
- 其它...

# 获取用户的消费项目（按整本书等统计）

> /user/consumeitems

| 参数  |       含义       |            可选值            |
| :---: | :--------------: | :--------------------------: |
| type  | 获取的消费项类型 | 见下，按逗号分隔，默认为全部 |
| page  |                  |              0               |
| size  |                  |              12              |

**type**

- novel: 小说
- comic: 漫画
- album: 有声小说

# 作者公告

> /user/authorAnnouncements

| 参数  | 含义  | 可选值 |
| :---: | :---: | :----: |
| page  |       |   0    |
| size  |       |   2    |

# 当前用户的火卷等信息

> /user/money

# 当前用户浏览过的漫画

> /user/comicvisits

# 当前用户浏览过的有声小说

> /user/albumvisits

# 推送信息

> /specialpush

|   参数    |   含义   |               可选值               |
| :-------: | :------: | :--------------------------------: |
| pushNames | 推送列表 | 见下，按逗号分隔，默认获取全部推送 |

**pushNames**

- home: 首页顶部 banner 推送，包括新书上架、征文活动与若干（一般 6 个）小说推送
- bigBrainPush: 脑洞推荐位
- contest2Push: 征文大赛作品推送
- boluoBannerPush: 菠萝包最新活动
- bottomButton: 重点征文大赛相关活动推送
- compositionbanner: 征文及相关活动推送
- merchPush: 周边推送
- entityDetailPush: 关于一些活动的细节的推送，例如悬浮窗推荐（不详）
- entityLastPagePush: 关于一些活动的细节的推送，例如尾页推荐（不详）
- homeTopRreshenPush: 有声专辑推荐（不详）
- interactNovel: 互动小说推荐（不详）
- myBanner: 主要活动推荐（不详）
- newSignInPush: 新签约小数推荐（不详）
- popup: 上架新书力荐（不详）
- vip: 精选专区 banner 推荐
- fanNovelHomePush: 不详
- firstChargePush: 不详
- homeBottomPush: 不详
- homeBottomTabPush: 不详
- homeBroadcastPush: 不详
- homeFloatPush: 不详
- homeHolidayPush: 不详
- announcementPush: 不详
- bookMarkPush: 不详
- chatNovelFinishGood: 不详
- chatNovelFinishHighPoint: 不详
- chatNovelHotDaily: 不详
- chatNovelHotEditor: 不详
- chatNovelHotLooking: 不详
- chatNovelHotType: 不详
- chatNovelHotbanners: 不详
- chatNovelNewHot: 不详
- chatNovelNewPotential: 不详
- chatNovelNewWin: 不详
- chatNovelPush: 不详
- chatnovelbannerPush: 不详
- comicCustomizePush: 不详

# 获取举报选项

> /reportOptions

# 获取当前 IP 属地

> /position

# 貌似是发放优惠卷的什么东西？不懂

> /entity/0/discountCoupons/qualification

|       参数       | 含义  | 可选值 |
| :--------------: | :---: | :----: |
|    entityType    |       |   1    |
| businessscenario |       |   0    |

# 获取所有的区号

> /countrycodes

# 不是非常确定，但应该是段评之类的东西（可以用 11098349 试试）

> /cmts/{cmtsId}

# 同上，反正是上面那个东西的回复

> /cmts/{cmtsId}/replys

| 参数  | 含义  | 可选值 |
| :---: | :---: | :----: |
| page  |       |   0    |
| size  |       |   20   |

# 不是非常确定，但应该是书评之类的东西（可以用 7417725 试试）

> /lcmts/{lcmtsId}

# 同上，反正是上面那个东西的回复

> /lcmts/{lcmtsId}/replys

| 参数  | 含义  | 可选值 |
| :---: | :---: | :----: |
| page  |       |   0    |
| size  |       |   20   |

# 获取用户阅读时的小说书签

> /bookmark

|  参数   |  含义   | 可选值 |
| :-----: | :-----: | :----: |
| novelId | 小说 ID |        |

# 无语子

> /advertisements

| 参数  | 含义  | 可选值 |
| :---: | :---: | :----: |
| page  |       |   0    |
| size  |       |   20   |
# 获取小说的衍生作品（漫画，有声小说）
/adpworks/novelId/{novelId}
query(
    expand: signlevel
)

# 无语子，就获取个图片资源而已

> /static/images

|  参数  | 含义  | 可选值 |
| :----: | :---: | :----: |
| fields |       |  见下  |

**fields**

- topIcons: 顶部图标
- bottomIcons: 底部图标

# 不晓得

> /chaps/0/{chapterId}/tsukkomis

|  参数  |   含义   |      可选值      |
| :----: | :------: | :--------------: |
| expand |          | 见下，以逗号分隔 |
|  sort  | 排序方式 |       hot        |
|  page  |          |        0         |
|  size  |          |        20        |
|  row   |          |        4         |

**expand**

- vipLevel
- avatar
- roleName
- widgets

# 无语子，是邀请好友转现金的广告

> /welfare/cfg

# 待处理

/widgets?page=0&size=20&sourceUid=4406079&type=badge&isactive=both&expand=collectdate%2Cisactived%2Ctotalcount%2CexpireDate&userowned=both
/workentities?page=4&size=12&expand=authorName%2CtypeName%2CsysTags%2Cintro%2Cdiscount%2CdiscountExpireDate

/user/welfare/storeitems/latest
/user/welfare/redpacket?deviceId=358523029890802
/user/welfare/income
/user/preOrderInfo?expand=intro%2CtypeName%2Ctags%2CsysTags
/user/Pockets?expand=novels%2Calbums%2Ccomics%2Cdiscount%2CdiscountExpireDate
/user/NovelViews/465469
/user/NovelViews
/user/novels?expand=ticket%2CapplyStatus%2Cintro%2CtypeName%2Csignlevel%2CsysTags%2CchapterCount%2CauditCover%2Ctopic%2CcustomTag
/user/novel/{novelId}/ticket/rankinglist?numMax=3&dateRange=1&expand=avatar
/user/novel/{novelId}/bonus/rankinglist?numMax=3&dateRange=1&expand=avatar
/user/generalCoupons?page=0&size=200&couponTypes=4%2C5%2C6&isUsed=not&isExpired=not&entityId=-1&sort=recordId&order=desc
/user/generalCoupons?page=0&size=200&couponTypes=4%2C5%2C6&isUsed=not&isExpired=not&entityId=-1&sort=amout&order=asc
/user/feeds?filter=followed&expand=novels%2Ccomics%2Calbums%2Ctags%2CsysTags%2CauthorName&page=0&size=12
/user/advertisements?deviceToken=72776e1f-9877-371f-ab5a-2b0bccefd275&page=0&size=20

/preOrderInfo?expand=intro%2CtypeName%2Ctags%2CsysTags&withExpiredPreOrder=false
/personalizedrecommend/user/novels?nid={novelId}&change=false&categoryId=0&page=0&size=3&expand=intro%2Ctags%2CsysTags&filter=
/personalizedrecommend/user/novels?categoryId=0&page=0&size=10&expand=homeflag%2CtypeName%2Cintro%2CsysTags&filter=dailyB
/feedbackuser?userIdentifer=72776e1f-9877-371f-ab5a-2b0bccefd275&channel=android
/feedback/fqa?index=0&size=6&isHot=2&isNew=1&category=-1
/novels/specialpushs?pushNames=hotpush&page=0&size=8&expand=sysTags%2Cdiscount%2CdiscountExpireDate%2ChomeFlag
/novels/{novelId}/lcmts?page=0&size=1&sort=addtime&charlen=140
/novels/{novelId}/lcmts?page=0&size=12&sort=addtime&charlen=140
/novels/{novelId}/cmts/barrage?typeId=3&isBig=2&page=3&size=60
/novels/{novelId}/Cmts?page=1&size=50&type=clear&sort=smart&replyUserId=0
/novels/{novelId}/Cmts?page=0&size=3&type=stick&sort=timeline&replyUserId=0
/novels/{novelId}/Cmts?page=0&size=2&type=stickandclear&sort=timeline&replyUserId=0
/novels/{novelId}/bonus/rank?numMax=50&dateRange=1
/novels/{novelId}/ticket/rank?numMax=50&dateRange=1
/novels/{novelId}/actpushes?filter=android&pageType=1
/novels/{novelId}/actpushes?filter=android&pageType=0
/novels?page=0&size=9&tid=25&categoryId=0&filter=recom&expand=discount%2CdiscountExpireDate

/users/{userId}/novels?expand=typeName%2CsysTags
/users/{userId}?expand=introduction%2CbigAvatar%2Cavatar%2CbackgroundPic%2CfansNum%2CfollowNum%2Cfollowyou%2Cyoufollow%2CverifyType%2CverifyInfo%2CavatarFrame%2Cyoublock%2Cwidgets
/users/{userId}/pocketEntities?page=2&size=60&expand=novels%2Ccomics%2Calbums%2CchatNovel
/users/{userId}/novels?expand=typeName%2CsysTags
/users/{userId}/dynamics?expand=novels%2Ccomics%2Calbums%2Ctags%2CsysTags%2CauthorName&page=0&size=20
