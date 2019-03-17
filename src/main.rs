use log::{error, Level};
use std::process;
use structopt::StructOpt;
use vdot::{logger, Args};

fn main() {
    // Setup logging to stdout and stderr.
    logger::init(Level::Info);

    // Parse the command line arguments into the Args struct, using structopt.
    let args = Args::from_args();

    // Run vdot!
    if let Err(err) = vdot::run(args) {
        error!("{}", err);
        process::exit(1);
    }
}
