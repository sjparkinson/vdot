# vdot

[![Build Status](https://travis-ci.org/sjparkinson/vdot.svg?branch=master)](https://travis-ci.org/sjparkinson/vdot)

Create your `.env` files using Vault.

> 🚧 Currently only works with [version 1 of the vault key/value secrets engine](https://www.vaultproject.io/docs/secrets/kv/kv-v1.html).

## Installation

**macOS** and **Linux**

```shell
curl https://raw.githubusercontent.com/sjparkinson/vdot/master/scripts/gh-install.sh \
| sh -s -- --git sjparkinson/vdot
```

**Cargo**

You can install `cargo` from https://www.rust-lang.org/tools/install.

```shell
cargo install vdot
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
