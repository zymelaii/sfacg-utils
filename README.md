<div align="center">

# sfacg-utils

sfacg-utils is an **API wrapper suite** for boluobao.

It provides a set of flexible interfaces to perform authorization, data acquisition, profile management and other operations easily.

[Getting started](#getting-started) •
[Contributors](#contributors)

</div>

## Getting started

Here is a minimum example for auth and data acquisition:

```python
from sfutils.proxy import SFBookProxy

account  = '<put-your-account-here>'
password = '<put-you-password-here>'

proxy = SFBookProxy().login(account, password)

proxy.me                    #!< view auth infomation
novelId = 123456
book = proxy.novel(novelId) #!< create novel instance
book.info                   #!< novel info
book.catalogue              #!< full catalogue of the book
book.volumes                #!< volumes of the book
volume = book.volume(2)     #!< create instance of the 3th volume
volume.info                 #!< volume info
volume.chapers              #!< chapters of the volume
```

## Contributors

![Contributors](https://contributors-img.web.app/image?repo=zymelaii/sfacg-utils&max=500)
