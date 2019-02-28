extern crate docopt;
extern crate reqwest;
extern crate vdot;

use docopt::Docopt;
use std::error::Error;
use vdot::Config;

const USAGE: &'static str = "
vdot

Create your .env file using Vault.

Usage:
  vdot <path>...
  vdot (-h | --help)
  vdot --version

Options:
  -h --help     Show this message.
  --version     Show the version of this program.
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(VERSION.to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    let config = Config::new(&args)?;

    vdot::run(config)
}
