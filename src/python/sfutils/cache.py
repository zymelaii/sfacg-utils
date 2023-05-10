from time import time
from copy import deepcopy

class CachedItem:
    def __init__(self):
        self.cache = {}

    def cache_load(self, key, validTime=360):
        if key not in self.cache:
            return (False, None)
        storeTime, value = self.cache[key]
        assert time() >= storeTime
        succeed = time() - storeTime < validTime
        return (succeed, deepcopy(value))

    def cache_store(self, key, value):
        self.cache[key] = (time(), deepcopy(value))
