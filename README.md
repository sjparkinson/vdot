# vdot

[![Build Status](https://travis-ci.org/sjparkinson/vdot.svg?branch=master)](https://travis-ci.org/sjparkinson/vdot)

Create your `.env` file using Vault.

## Usage

```
vdot

Usage:
  vdot <path>...
  vdot -h | --help
  vdot --version

Options:
  -h --help     Show this screen.
  --version     Show version.
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
