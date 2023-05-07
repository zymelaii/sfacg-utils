import csv
import requests
from random import choice
from urllib.parse import unquote
from bs4 import BeautifulSoup
from time import time
from copy import deepcopy

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

class ChapterItem:
    _keyList = None

    def __init__(self, novel, cid, fetchData=True):
        if ChapterItem._keyList is None:
            with open('keylist.csv') as f:
                ChapterItem._keyList = [e for e in csv.DictReader(f)]

        self.novel = novel
        self.chapterId = cid
        self._mark_as_dirty()

        if fetchData:
            self.fetch()

    def _mark_as_dirty(self):
        self.novelId = None
        self.volumeId = None
        self.chapterName = None
        self.pubTime = None
        self.updateTime = None
        self.isVip = None

        self.content = None

    def __str__(self):
        return f"{'{'}'ChapterName': {self.chapterName}, 'ChapterID': {self.chapterId}{'}'}"

    def __repr__(self):
        return str(self)

    def _getSignKey(self):
        return choice(ChapterItem._keyList)

    def _fetch(self):
        global cookies

        signkey = self._getSignKey()
        SFSecurity = '&'.join(map(lambda e: f'{e[0]}={e[1]}', signkey.items()))
        appversion = '4.7.46(android;25)'

        headers = {
            'Accept-Charset': 'UTF-8',
            'Authorization': 'Basic YW5kcm9pZHVzZXI6MWEjJDUxLXl0Njk7KkFjdkBxeHE=',
            'Accept': 'application/vnd.sfacg.api+json;version=1',
            'User-Agent': f'boluobao/{appversion}/TENCENT/{signkey["devicetoken"].lower()}',
            'SFSecurity': SFSecurity,
        }

        url = f'https://api.sfacg.com/Chaps/{self.chapterId}'
        resp = requests.get(
            url,
            params={
                'expand': 'content,needFireMoney,originNeedFireMoney,tsukkomi,chatlines',
                'autoOrder': False
            },
            headers=headers,
            cookies=cookies,
        )

        return resp.json()['data']

    def fetch(self):
        if self.content is None:
            resp = self._fetch()

            self.novelId = resp['novelId']
            self.volumeId = resp['volumeId']
            self.chapterName = resp['title']
            self.pubTime = resp['addTime']
            self.updateTime = resp['updateTime']
            self.isVip = resp['isVip']
            self.content = resp['expand']['content']

class Novel:
    def __init__(self, nid):
        self.novelId = nid
        self._mark_as_dirty()

    def _mark_as_dirty(self):
        self.volumeCache = None

    @property
    def volumes(self):
        if self.volumeCache is None:
            url = 'https://book.sfacg.com/ajax/ashx/Common.ashx'
            resp = requests.post(
                url,
                params={
                    'op': 'getPlanNovelInfo',
                },
                data={
                    'nid': self.novelId,
                },
                headers=headers
            )
            self.volumeCache = resp.json()['novel']['VolumeSet']
        return deepcopy(self.volumeCache)

    def chaptersOf(self, id):
        volumeIdSet = map(lambda e: e['VolumeID'], self.volumes)
        if id not in volumeIdSet:
            if not (id >= 0 and id < len(self.volumes)):
                raise IndexError('volume index out of range')
            id = self.volumes[id]['VolumeID']
        url = 'https://i.sfacg.com/ajax/ashx/GetChapterInfo.ashx'
        resp = requests.post(
            url,
            params={
                'vid': id,
            },
            headers=headers
        )
        result = sorted(resp.json(), key=lambda e: e['co'])
        result = map(lambda e: {
            'ChapterName': unquote(e['ct']),
            'ChapterID': e['cid'],
        }, result)
        result = map(lambda e: ChapterItem(self, e['ChapterID'], fetchData=False), result)
        return list(result)
