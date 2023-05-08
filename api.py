import csv
import requests
import pickle
from random import choice
from urllib.parse import unquote
from bs4 import BeautifulSoup
from time import time
from datetime import datetime
from copy import deepcopy

<<<<<<< HEAD
headers = {
    'Host': 'book.sfacg.com',
    'Connection': 'keep-alive',
    'Cache-Control': 'max-age=0',
    'Upgrade-Insecure-Requests': '1',
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.132 Safari/537.36',
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3',
    'Referer': 'https://book.sfacg.com/',
    'Accept-Encoding': 'gzip, deflate',
    'Accept-Charset': 'UTF-8',
    'Accept-Language': 'zh-CN,zh;q=0.9',
}
cookies = {
    '.SFCommunity': '<token>', # replace <token> with your .SFCommunity cookie
}
=======
'''
Cookie 的 .SFCommunity 和 seession_APP 是必须的
可从 Android 的 /data/data/com.sfacg/app_webview/Default/Cookie 获取
其为 SQLite Database
'''
>>>>>>> a361a8b (feat: integrate official api)

with open('secrets.csv') as f:
    __SECRETPOOL__ = [e for e in csv.DictReader(f)]

__SFCOMMUNITY__ = 'FIXME: Cookie ".SFCommunity"'
__SESSIONAPP__ = 'FIXME: Cookie "session_APP"'

class api:
    def headers():
        signkey = choice(__SECRETPOOL__)
        SFSecurity = '&'.join(map(lambda e: f'{e[0]}={e[1]}', signkey.items()))
        appversion = '4.7.46(android;25)'
        headers = {
            'Accept-Charset': 'UTF-8',
            'Authorization': 'Basic YW5kcm9pZHVzZXI6MWEjJDUxLXl0Njk7KkFjdkBxeHE=',
            'Accept': 'application/vnd.sfacg.api+json;version=1',
            'User-Agent': f'boluobao/{appversion}/TENCENT/{signkey["devicetoken"].lower()}',
            'SFSecurity': SFSecurity,
        }
        return headers

    def cookies():
        cookies = {
           '.SFCommunity': __SFCOMMUNITY__,
            'session_APP': __SESSIONAPP__,
        }
        return cookies

    '''
    api-url: https://api.sfacg.com/users
    params:
        - expand: [verifyType | avatarFrame | widgets],...
        - uids: <userid>,...
    '''
    def users(uids, expand=[]):
        url = 'https://api.sfacg.com/users'
        resp = requests.get(
            url,
            params={
                'expand': ','.join(expand),
                'uids': ','.join(map(str, uids)),
            },
            headers=api.headers(),
        )
        return resp.json()

    '''
    api-url: 'https://api.sfacg.com/users/<user-id>
    params:
        - expand: [introduction | bigAvatar | avatar |
            backgroundPic | fansNum | followNum |
            followyou | youfollow | verifyType |
            verifyInfo | avatarFrame | youblock |
            widgets],...'
    '''
    def userInfo(uid, expand=[]):
        url = f'https://api.sfacg.com/users/{uid}'
        resp = requests.get(
            url,
            params={
                'expand': ','.join(expand),
            },
            headers=api.headers(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/user/signInfo
    '''
    def signInfo():
        url = 'https://api.sfacg.com/user/signInfo'
        resp = requests.get(
            url,
            headers=api.headers(),
            cookies=api.cookies(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/Chaps/<chapterId>
    params:
        - expand: [content | needFireMoney | originNeedFireMoney |
            tsukkomi | chatlines],...
        - autoOrder: [true | false]
    note: vip 章节获取 content 需要登录
    '''
    def chapter(chapterId, expand=[], autoOrder=False):
        url = f'https://api.sfacg.com/Chaps/{chapterId}'
        resp = requests.get(
            url,
            params={
                'expand': ','.join(expand),
                'autoOrder': autoOrder,
            },
            headers=api.headers(),
            cookies=api.cookies(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/novels/<novelId>
    params:
        - expand: [chapterCount | bigBgBanner | bigNovelCover |
            typeName | intro | fav |
            ticket | pointCount | tags |
            sysTags | signlevel | discount |
            discountExpireDate | totalNeedFireMoney | rankinglist |
            originTotalNeedFireMoney | firstchapter | latestchapter |
            latestcommentdate | essaytag | auditCover |
            preOrderInfo | customTag | topic |
            unauditedCustomtag],...
    '''
    def novelInfo(novelId, expand=[]):
        url = f'https://api.sfacg.com/novels/{novelId}'
        resp = requests.get(
            url,
            params={
                'expand': ','.join(expand),
            },
            headers=api.headers(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/novels/<novelId>/dirs
    params:
        - expand: [originNeedFireMoney],...
    '''
    def catalogueOf(novelId, expand=[]):
        url = f'https://api.sfacg.com/novels/{novelId}/dirs'
        resp = requests.get(
            url,
            params={
                'expand': ','.join(expand),
            },
            headers=api.headers(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/user
    params:
        - expand: [vipInfo | welfareCoin | isRealNameAuth |
            changeNickNameInfo | welfareMoney | redpacketCode |
            useWelfaresys | usedRedpacketCode | hasOrderChapWithFireMoney |
            hasUnlockChapWithAd | hasActiveUnlockChapWithAd | hasOrderedVipChaps |
            hasPaidFirstTime]
    note: 需要登录
    '''
    def user(expand=[]):
        url = 'https://api.sfacg.com/user'
        resp = requests.get(
            url,
            params={
                'expand': ','.join(expand),
            },
            headers=api.headers(),
            cookies=api.cookies(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/androidcfg
    '''
    def androidConfig():
        url = 'https://api.sfacg.com/androidcfg'
        resp = requests.get(
            url,
            headers=api.headers(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/authors/<authorId>/novels
    '''
    def novelsOf(authorId):
        url = f'https://api.sfacg.com/authors/{authorId}/novels'
        resp = requests.get(
            url,
            headers=api.headers(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/authors
    params:
        - authorId: <authorId>
        - expand: [youfollow | fansNum]
    note: 未提供 .SFCommunity 时 youfollow 恒为 false
    '''
    def authorInfo(authorId, expand=[]):
        url = 'https://api.sfacg.com/authors'
        resp = requests.get(
            url,
            params={
                'authorId': authorId,
                'expand': ','.join(expand),
            },
            headers=api.headers(),
            cookies=api.cookies(),
        )
        return resp.json()

    '''
    api-url: https://api.sfacg.com/user/badge
    params:
        - vipDateTime: <vipDateTime>
        - badgeAddDateTime: <badgeAddDateTime>
        - channle: [Android | ...]
        - userIdentifer: <deviceToken>
    note: 时间为 ISO 格式，精确到秒
    '''
    def badge(
        vipDateTime='',
        badgeAddDateTime=datetime.now().isoformat(timespec='seconds'),
        channel='Android',
        userIdentifer=choice(__SECRETPOOL__)['devicetoken'].lower()
    ):
        url = 'https://api.sfacg.com/user/badge'
        resp = requests.get(
            url,
            params={
                'vipDateTime': vipDateTime,
                'badgeAddDateTime': badgeAddDateTime,
                'channel': channel,
                'userIdentifer': userIdentifer,
            },
            headers=api.headers(),
        )
        return resp.json()
