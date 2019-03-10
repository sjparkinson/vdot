# vdot

[![Build Status](https://travis-ci.org/sjparkinson/vdot.svg?branch=master)](https://travis-ci.org/sjparkinson/vdot)

Create your `.env` files using Vault.

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

**KV Secrets Engine Version 2**

```bash
$ vault login
$ vault kv put secret/foo-bar ENV=production LOG_LEVEL=info
$ vault kv put secret/fizz-buzz LOG_LEVEL=debug
$ vdot secret/data/foo-bar secret/data/fizz-buzz
vdot: saved 2 environment variables to .env
$ cat .env
ENV=production
LOG_LEVEL=debug
```

**KV Secrets Engine Version 1**

```bash
$ vault login
$ vault kv put secret/foo-bar ENV=production LOG_LEVEL=info
$ vdot secret/foo-bar
vdot: saved 2 environment variables to .env
$ cat .env
ENV=production
LOG_LEVEL=info
```