# vdot

[![Build Status](https://travis-ci.org/sjparkinson/vdot.svg?branch=master)](https://travis-ci.org/sjparkinson/vdot)

Create your `.env` files using Vault.

## Installation

**macOS** and **Linux**

This script will download the latest release from GitHub and install `vdot` under `/usr/local/bin`.

```shell
curl https://gist.githubusercontent.com/sjparkinson/327dc78c60ab81a06c946630b4288910/raw/crate-gh-install.sh \
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
  vdot [-v] <path>...
  vdot (-h | --help)
  vdot --version

Options:
  -h --help      Show this message.
  --version      Show the version of this program.
  -v, --verbose  Use verbose output.
```

**KV Secrets Engine Version 2**

```bash
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
$ vault kv put secret/foo-bar LOG_LEVEL=info
$ vdot secret/foo-bar
vdot: saved 1 environment variable to .env
$ cat .env
LOG_LEVEL=info
```