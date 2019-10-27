# vdot

Create your `.env` files using HashiCorp Vault.

> 🔮 **Want to start a process with Vault?** Consider using [HashiCorp's `envconsul`](https://github.com/hashicorp/envconsul).

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

You can download executables for macOS, Linux, and Windows from https://github.com/sjparkinson/vdot/releases/latest.

## Usage

```
vdot

USAGE:
    vdot [FLAGS] [OPTIONS] <PATH>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -o, --output <path>              Write to the given file [default: .env]
        --kv <version>               Version of the key value secrets engine [default: 2]  [possible values: 1, 2]
        --vault-token <token>        Vault token used to authenticate requests [env: VAULT_TOKEN]
        --vault-address <address>    Vault server address [env: VAULT_ADDR=http://127.0.0.1:8200]

ARGS:
    <path>...    Path to the Vault secrets
```

The following assumes you are [using **version two** of Vault's key-value secret engine](https://www.vaultproject.io/docs/secrets/kv/index.html). Check out `vdot --help` for more information.

```bash
$ vault kv put secret/foo-bar ENV=production LOG_LEVEL=info
$ vault kv put secret/fizz-buzz LOG_LEVEL=debug
$ vdot --vault-token "$(cat ~/.vault-token)" secret/foo-bar secret/fizz-buzz
info: saved 2 environment variables to .env
$ cat .env
ENV=production
LOG_LEVEL=info
```

## Environment Variables

Instead of passing in the `--vault-*` options, you can define them as environmnent variables.

The [Vault CLI defines that](https://www.vaultproject.io/docs/commands/index.html#environment-variables) `VAULT_TOKEN` and `VAULT_ADDR` can be used. These two environment variables are also supported by vdot.
