<div align="center">

# sfacg-utils

sfacg-utils is an **API wrapper suite** for boluobao.

It provides a set of flexible interfaces to perform authorization, data acquisition, profile management and other operations easily.

[Learn about sfutils](#learn-about-sfutils) •
[Installation](#installation) •
[Getting started](#getting-started) •
[About privacy](#about-privacy) •
[Contributors](#contributors)

</div>

## Learn about sfutils

It is not only the name of the module provided by this project, but also a powerful cli written in Rust for personal matters on boluobao.

As an api-based program, sfutils supports features as co-login, custom actions, data backup, etc.

## Installation

You can simply build and install from source via cargo:

```shell
cargo install --path . --bin sfutils
```

In addition, the latest release is also available on our repo page.

Package for Python can be found under `src/python`. You can do whatever you want with it. But given that we spent more time and effort on the version of Rust, it might be a better choice to use the rust module for secondary development.

## Getting started

```shell
$ sfutils help
An efficent yet powerful cli for boluobao

Usage: sfutils <COMMAND>

Commands:
  auth  Authenticate sfutils with boluobao
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

You can simply login your boluobao through subcommand **auth**. Credentials will be saved locally by default after successful authentication.

```shell
sfutils auth login -u <account> -p <password>
```

All the stored credentials can be found through `sfutils auth status list`. Assuming that **foobar** is your nickname on boluobao and it displays in the output list, then your can simply login through your username:

```shell
sfutils auth login -U foobar
```

Similarly, credentials can be destroyed through `logout`:

```shell
sfutils auth logout foobar
```

See help information for more usage.

## Usage for sfutils package in Python

Here is a minimum example for auth and data acquisition:

```python
from sfutils.proxy import SFBookProxy

account  = '<put-your-account-here>'
password = '<put-you-password-here>'

proxy = SFBookProxy().login(account, password)

proxy.me                        #!< view auth infomation
novelId = 123456
book = proxy.novel(novelId)     #!< create novel instance
book.info                       #!< novel info
book.catalogue                  #!< full catalogue of the book
book.volumes                    #!< volumes of the book
volume = book.volume(2)         #!< create instance of the 3th volume
volume.info                     #!< volume info
volume.chapers                  #!< chapters of the volume
chapter = volume.chapter(-7)    #!< create instance of the penultimate chapter 7
chapter.info                    #!< chapter info
chapter.content                 #!< chapter content
```

## About privacy

All data involving personal privacy will be kept on your own PC, and you can even force it not to store them with certain parameters.

It should be noted, however, that some of the data you obtain through the cli may relate to the intellectual property and privacy agreements of boluobao, so you should keep your authentication credentials safe or destroy them from time to time, and under no circumstances should the data obtained be distributed in any form, especially regarding the content of the novel.

## Contributors

See [Contributing](https://github.com/zymelaii/sfacg-utils/graphs/contributors) for details. Thanks to all the people who already contributed!

![Contributors](https://contributors-img.web.app/image?repo=zymelaii/sfacg-utils&max=500)
