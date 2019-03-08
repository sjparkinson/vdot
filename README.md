# vdot

[![Build Status](https://travis-ci.org/sjparkinson/vdot.svg?branch=master)](https://travis-ci.org/sjparkinson/vdot) [![Build status](https://ci.appveyor.com/api/projects/status/uah7hho8x7bitv2a/branch/master?svg=true)](https://ci.appveyor.com/project/sjparkinson/vdot/branch/master)

Create your `.env` files using Vault.

> 🚧 Currently only works with [version 1 of the vault key/value secrets engine](https://www.vaultproject.io/docs/secrets/kv/kv-v1.html).

## Installation

**macOS**

```shell
curl -LSfs https://japaric.github.io/trust/install.sh | \
  sh -s -- --git sjparkinson/vdot --target x86_64-unknown-linux-musl --to /usr/local/bin
```

**Linux**

```shell
curl -LSfs https://japaric.github.io/trust/install.sh | \
  sh -s -- --git sjparkinson/vdot --target x86_64-apple-darwin --to /usr/local/bin
```

**Windows**

You can [download `vdot.exe` from the latest release](https://github.com/sjparkinson/vdot/releases/latest). You will also need to install the [Visual C++ Redistributable for Visual Studio 2015](https://www.microsoft.com/en-us/download/details.aspx?id=48145).

## Usage

```
vdot

Create your .env files using Vault.

Usage:
  vdot <path>...
  vdot (-h | --help)
  vdot --version

Options:
  -h --help     Show this message.
  --version     Show the version of this program.
```

```shell
$ vault login
$ vault write secret/foo-bar ENV=production
$ vault write secret/fizz-buzz LOG_LEVEL=info
$ vdot secret/foo-bar secret/fizz-buzz
Saved 2 environment variables to .env
$ cat .env
ENV=production
LOG_LEVEL=info
```
