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
use log::Level;
use std::process;

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

    if let Err(err) = vdot::run(&args) {
        error!("{}", err);
        process::exit(1);
    }
}
