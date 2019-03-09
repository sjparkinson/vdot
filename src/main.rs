extern crate directories;
extern crate docopt;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate vdot;

use docopt::Docopt;
use std::process;
use vdot::Config;

const USAGE: &str = "
vdot

Create your .env files using Vault.

Usage:
  vdot <path>...
  vdot (-h | --help)
  vdot --version

Options:
  -h --help     Show this message.
  --version     Show the version of this program.
";

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(VERSION.to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = vdot::run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
