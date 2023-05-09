import json
from . import constants
from .cache import CachedItem

import requests
import uuid
import hashlib
from time import time
from copy import deepcopy

class SFBookMetaProxy(CachedItem):
    def __init__(self, **kwargs):
        super(SFBookMetaProxy, self).__init__()
        self.token = kwargs.get('token', '')
        self.session = kwargs.get('session', '')
        self.devicetoken = kwargs.get('devicetoken', str(uuid.uuid1())).lower()
        self.appversion = kwargs.get('appversion', list(constants.appkey_dict.keys())[0])
        self.channel = kwargs.get('channel', 'HomePage')
        self.logon = False

    def ua(self):
        return f'boluobao/{self.appversion}/{self.channel}/{self.devicetoken}'

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

    # NOTE: defaultly for GET http method
    def headers(self, extraHeaders=None):
        headers = {
            'Accept-Charset': 'UTF-8',
            'Authorization': constants.authorization,
            'Accept': 'application/vnd.sfacg.api+json;version=1',
            'User-Agent': self.ua(),
            'SFSecurity': self.security(),
        }
        if extraHeaders is not None:
            assert isinstance(extraHeaders, dict)
            for key, value in extraHeaders.items():
                result = next(filter(lambda e: e.lower() == key.lower(), headers.keys()), None)
                headers[key if result is None else result] = value
        return headers

    def cookies(self):
        cookies = {
            '.SFCommunity': self.token,
            'session_APP': self.session,
        }
        return cookies

    def _invoke_api(self, method, apiUri, asJson=True, **kwargs):
        kwargs.setdefault('headers', self.headers())
        kwargs.setdefault('cookies', self.cookies())
        url = f'https://api.sfacg.com/{apiUri}'
        session = requests.Session()
        req = requests.Request(method, url, **kwargs)
        resp = session.send(req.prepare())
        return resp.json() if asJson else resp

    def _try_login(self, account, password):
        assert account and password
        url = 'https://api.sfacg.com/sessions'
        params = {
            'username': account,
            'password': password,
        }
        data = json.dumps(params)
        headers = self.headers({'Content-Type': 'application/json'})
        resp = self._invoke_api('POST', 'sessions', data=data, headers=headers, asJson=False)
        if resp.status_code != 200:
            return None
        result = {
            'token': resp.cookies['.SFCommunity'],
            'session': resp.cookies['session_APP'],
        }
        return result

    def login(self, account=None, password=None, **kwargs):
        # TODO: handle expired time
        # reject login operations in the logon state unless "force" is specified
        force = kwargs.get('force', False)
        if self.logon and force:
            self.logout()
        if not self.logon:
            # login with account & password
            if account and password:
                result = self._try_login(account, password)
                if result is not None:
                    self.token = result['token']
                    self.session = result['session']
            # specify manually
            if len(self.token) == 0 or len(self.session) == 0:
                self.token = kwargs.get('token', self.token)
                self.session = kwargs.get('session', self.session)
            # verify auth
            if self.me is not None:
                self.logon = True
        return self

    def logout(self):
        self.logon = False
        return self

    @property
    def me(self, expand=[]):
        succeed, value = self.cache_load('me')
        if succeed:
            return value
        # TODO: merge expand info
        resp = self._invoke_api('GET', 'user', params={'expand': ','.join(expand)})
        value = resp['data'] if resp['status']['httpCode'] == 200 else value
        self.cache_store('me', value)
        return value

class SFBookProxy(SFBookMetaProxy):
    def __init__(self, **kwargs):
        super(SFBookProxy, self).__init__(**kwargs)

    '''
    NovelAPI.Controllers.NovelAPIController
    System.Net.Http.HttpResponseMessage GetNovel(Int32, System.String)
    '''

    def GetNovel(self, novelId, expand=[]):
        return self._invoke_api('GET', f'novels/{novelId}', params={'expand': ','.join(expand)})

    '''
    NovelAPI.Controllers.NovelAPIController
    System.Net.Http.HttpResponseMessage GetNovelDirs(Int32, Int32)
    '''

    def GetNovelDirs(self, novelId, expand=[]):
        return self._invoke_api('GET', f'novels/{novelId}/dirs', params={'expand': ','.join(expand)})

    def novel(self, novelId):
        assert (isinstance(novelId, int) and novelId < 2 ** 31 - 1)
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
