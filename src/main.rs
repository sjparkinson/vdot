use log::{error, Level};
use std::process;
use structopt::StructOpt;
use vdot::{logger, Args};

fn main() {
    let args: Args = Args::from_args();

    // Setup logging to stdout and stderr.
    match args.verbose {
        0 => logger::init(Level::Info),
        1 => logger::init(Level::Debug),
        _ => logger::init(Level::Trace),
    }

    if let Err(err) = vdot::run(args) {
        error!("{}", err);
        process::exit(1);
    }
}