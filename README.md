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

You can also download the executable manually from https://github.com/sjparkinson/vdot/releases/latest.

**Cargo**

> You can install `cargo` from https://www.rust-lang.org/tools/install.

```shell
cargo install vdot
```

## Usage

```
vdot

USAGE:
    vdot [FLAGS] <OPTIONS> <PATH>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
        --vault-address <vault_address>    The URL to access Vault. [env: VAULT_ADDR]
        --vault-token <vault_token>        Your Vault token. [env: VAULT_TOKEN]

ARGS:
    <PATH>...    Vault paths
```

**Key-Value Version 2**

```bash
$ vault kv put secret/foo-bar ENV=production LOG_LEVEL=info
$ vault kv put secret/fizz-buzz LOG_LEVEL=debug
$ vdot --vault-address "$VAULT_ADDR" --vault-token "$(cat ~/.vault-token)" secret/data/foo-bar secret/data/fizz-buzz
info: saved 2 environment variables to .env
$ cat .env
ENV=production
LOG_LEVEL=debug
```

**Key-Value Version 1**

```bash
$ vault kv put secret/foo-bar LOG_LEVEL=info
$ vdot --vault-address "$VAULT_ADDR" --vault-token "$(cat ~/.vault-token)" secret/foo-bar
info: saved 1 environment variable to .env
$ cat .env
LOG_LEVEL=info
```

## Environment Variables

Instead of passing in the `--vault-*` options, you can define them as environmnent variables.

The [Vault CLI defines that](https://www.vaultproject.io/docs/commands/index.html#environment-variables) `VAULT_TOKEN` and `VAULT_ADDR` can be used.

These two environment variables are also supported by vdot.
