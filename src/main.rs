#[macro_use]
extern crate log;
extern crate directories;
extern crate docopt;
extern crate failure;
extern crate loggerv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate vdot;

use docopt::Docopt;
use failure::Error;
use log::Level;
use std::process;
use vdot::Config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
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
";

fn main() {
    let version = Some(VERSION.to_string());
    let args = Docopt::new(USAGE)
        .and_then(|docopt| docopt.version(version).parse())
        .unwrap_or_else(|err| err.exit());

    if args.get_bool("--verbose") {
        loggerv::init_with_level(Level::Debug).unwrap();
    } else {
        loggerv::init_with_level(Level::Info).unwrap();
    }

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => return error(&err),
    };

    if let Err(err) = vdot::run(config) {
        error(&err);
    }
}

fn error(err: &Error) {
    error!("{}", err);
    process::exit(1);
}
