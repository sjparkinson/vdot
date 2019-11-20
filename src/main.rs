use log::{error, Level};
use std::process;
use structopt::StructOpt;
use vdot::{logger, Args};

fn main() {
    // Parse the command line inputs into an instance of `Args`.
    let args = Args::from_args();

    // Setup logging to stdout and stderr.
    if args.verbose {
        logger::init(Level::Debug)
    } else {
        logger::init(Level::Info)
    }

    // Run vdot!
    if let Err(err) = vdot::run(args) {
        error!("{}", err);
        process::exit(1);
    }
}
