from . import constants
from .cache import CachedItem

import requests
import uuid
import hashlib
from time import time
from copy import deepcopy

class SFBookMetaProxy:
    def __init__(self, **kwargs):
        '''
        API Cookie .SFCommunity
        用户标识 TOKEN
        '''
        self.token = kwargs.get('token', '')

        '''
        API Cookie session_APP
        使用与登录用户相关的 API 必须提供有效的 APP Session
        session_APP 可从 /data/data/com.sfacg/app_webview/Default/Cookie 获取
        '''
        self.session = kwargs.get('session', '')

        '''
        设备 TOKEN
        由设备唯一确认，若未提供则用随机生成的 UUID 代替
        '''
        self.devicetoken = kwargs.get('devicetoken', str(uuid.uuid1())).lower()
        try:
            foobar = uuid.UUID(self.devicetoken)
        except ValueError:
            assert(False)

        '''
        菠萝包轻小说 App 版本
        与 APPKEY 直接相关，不同的版本有不同的 APPKEY
        '''
        self.appversion = kwargs.get('appversion', list(constants.appkey_dict.keys())[0])
        assert(self.appversion in constants.appkey_dict.keys())

        '''
        菠萝包轻小说 App 登陆渠道
        '''
        self.channel = kwargs.get('channel', 'HomePage')

        self.logon = False

    '''
    API 请求标头 User-Agent
    User-Agent 指定的 appversion 必须与使用的 APPKEY 相配对
    '''
    def ua(self):
        return f'boluobao/{self.appversion}/{self.channel}/{self.devicetoken}'

    '''
    API 请求标头 SFSecurity
    '''
    def security(self):
        appkey = constants.appkey_dict[self.appversion]
        params = {
            'nonce': str(uuid.uuid1()).upper(),
            'timestamp': int(time() * 1e3),
            'devicetoken': self.devicetoken.upper(),
        }
        origin = f"{params['nonce']}{params['timestamp']}{params['devicetoken']}{appkey}"
        encryptor = hashlib.md5()
        encryptor.update(origin.encode('utf-8'))
        params['sign'] = encryptor.hexdigest().upper()
        return '&'.join(map(lambda e: f'{e[0]}={e[1]}', params.items()))

    '''
    API 请求标头
    仅提供 GET 请求的标头
    '''
    def headers(self):
        headers = {
            'Accept-Charset': 'UTF-8',
            'Authorization': constants.authorization,
            'Accept': 'application/vnd.sfacg.api+json;version=1',
            'User-Agent': self.ua(),
            'SFSecurity': self.security(),
        }
        return headers

    '''
    API 请求 Cookies
    '''
    def cookies(self):
        cookies = {
            '.SFCommunity': self.token,
            'session_APP': self.session,
        }
        return cookies

    def _invoke_api(self, subpath, params=None):
        url = f'https://api.sfacg.com/{subpath}'
        resp = requests.get(url, params=params, headers=self.headers(), cookies=self.cookies())
        return resp.json()

    def login(self, username='', password='', **kwargs):
        #! FIXME: 暂不支持账户密码登录
        if not self.logon:
            if len(self.token) == 0 or len(self.session) == 0:
                self.token = kwargs.get('token', self.token)
                self.session = kwargs.get('session', self.session)
            if self.me is not None:
                self.logon = True
        return self

    def logout(self):
        self.logon = False
        return self

    @property
    def me(self, expand=[]):
        resp = self._invoke_api('user', params={
            'expand': ','.join(expand)
        })
        return resp['data'] if resp['status']['httpCode'] == 200 else None

class SFBookProxy(SFBookMetaProxy):
    def __init__(self, **kwargs):
        super(SFBookProxy, self).__init__(**kwargs)

    '''
    NovelAPI.Controllers.NovelAPIController
    System.Net.Http.HttpResponseMessage GetNovel(Int32, System.String)
    '''
    def GetNovel(self, novelId, expand=[]):
        return self._invoke_api(f'novels/{novelId}', params={'expand': ','.join(expand)})

    '''
    NovelAPI.Controllers.NovelAPIController
    System.Net.Http.HttpResponseMessage GetNovelDirs(Int32, Int32)
    '''
    def GetNovelDirs(self, novelId, expand=[]):
        return self._invoke_api(f'novels/{novelId}/dirs', params={'expand': ','.join(expand)})

    def novel(self, novelId):
        assert(isinstance(novelId, int) and novelId < 2 ** 31 - 1)
        resp = self.GetNovel(novelId)
        return SFBookProxy.NovelWrapper(self, novelId, resp['data']) if resp['status']['httpCode'] == 200 else None

    class NovelWrapper(CachedItem):
        def __init__(self, delegator, novelId, info):
            super(SFBookProxy.NovelWrapper, self).__init__()
            self.delegator = delegator
            self.novelId = novelId
            self.cache_store('info', info)

        def volume(self, volumeId):
            totalVolumes = len(self.volumes)
            if volumeId >= 0 and volumeId < totalVolumes:
                volumeId = self.volumes[volumeId]['volumeId']
            if volumeId < 0 and volumeId >= -totalVolumes:
                volumeId = self.volumes[totalVolumes + volumeId]['volumeId']
            result = next(filter(lambda e: e['volumeId'] == volumeId, self.volumes), None)
            return SFBookProxy.VolumeWrapper(self, volumeId, result) if result is not None else None

        @property
        def info(self):
            succeed, value = self.cache_load('info')
            if succeed:
                return value
            resp = self.delegator.GetNovel(self.novelId)
            value = resp if resp['status']['httpCode'] == 200 else value
            self.cache_store('info', value)
            return value

        @property
        def catalogue(self):
            succeed, value = self.cache_load('catalogue')
            if succeed:
                return value
            resp = self.delegator.GetNovelDirs(self.novelId)
            value = resp['data']['volumeList']
            self.cache_store('catalogue', value)
            return value

        @property
        def volumes(self):
            volumes = sorted(self.catalogue, key=lambda e: e['sno'])
            volumes = map(lambda e: {
                'volumeId': e['volumeId'],
                'title': e['title'],
                'chapterCount': len(e['chapterList']),
                'charCount': sum(map(lambda e: e['charCount'], e['chapterList'])),
            }, volumes)
            return list(volumes)

    class VolumeWrapper(CachedItem):
        def __init__(self, novel, volumeId, info):
            super(SFBookProxy.VolumeWrapper, self).__init__()
            self.delegator = novel.delegator
            self.novel = novel
            self.volumeId = volumeId
            self.cache_store('info', info)

        @property
        def info(self):
            succeed, value = self.cache_load('info')
            if succeed:
                return value
            resp = next(filter(lambda e: e['volumeId'] == self, self.novel.volumes), None)
            value = resp if resp is not None else value
            self.cache_store('info', value)
            return value

        @property
        def chapters(self):
            result = next(filter(lambda e: e['volumeId'] == self.volumeId, self.novel.catalogue), None)
            return result['chapterList'] if result is not None else None
