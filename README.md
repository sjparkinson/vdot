# vdot

[![Build Status](https://travis-ci.org/sjparkinson/vdot.svg?branch=master)](https://travis-ci.org/sjparkinson/vdot)

Create your `.env` files and start processes using Vault.

## Installation

**Homebrew** and **Linuxbrew**

> You can install `brew` from https://brew.sh.

```shell
brew tap sjparkinson/vdot https://github.com/sjparkinson/vdot
brew install vdot
```

**Cargo**

> You can install `cargo` from https://www.rust-lang.org/tools/install.

```shell
cargo install vdot
```

**Download**

You can download `vdot` executables for macOS and Linux from https://github.com/sjparkinson/vdot/releases/latest.

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
    -c, --command <command>                Command to spawn
        --vault-address <vault_address>    Vault server address [env: VAULT_ADDR]
        --vault-token <vault_token>        Vault token used to authenticate requests [env: VAULT_TOKEN]

ARGS:
    <PATH>...    Path to the Vault secrets
```

The following assumes you are [using **version two** of Vault's key-value secret engine](https://www.vaultproject.io/docs/secrets/kv/index.html). Check out `vdot --help` for more information.

```bash
$ vault kv put secret/foo-bar ENV=production LOG_LEVEL=info
$ vault kv put secret/fizz-buzz LOG_LEVEL=debug
$ vdot --vault-token "$(cat ~/.vault-token)" secret/data/foo-bar secret/data/fizz-buzz
info: saved 2 environment variables to .env
$ cat .env
ENV=production
LOG_LEVEL=info
```

## Environment Variables

Instead of passing in the `--vault-*` options, you can define them as environmnent variables.

The [Vault CLI defines that](https://www.vaultproject.io/docs/commands/index.html#environment-variables) `VAULT_TOKEN` and `VAULT_ADDR` can be used. These two environment variables are also supported by vdot.
