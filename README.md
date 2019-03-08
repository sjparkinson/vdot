# vdot

[![Build Status](https://travis-ci.org/sjparkinson/vdot.svg?branch=master)](https://travis-ci.org/sjparkinson/vdot)

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
