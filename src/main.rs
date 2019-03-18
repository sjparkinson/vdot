use log::{error, Level};
use std::process;
use structopt::StructOpt;
use vdot::{logger, Args};

fn main() {
    // Parse the command line inputs into an instance of `Args`.
    let args = Args::from_args();

    // Convert the u8 into a `Level`.
    let log_level = match args.verbose {
        0 => Level::Info,
        _ => Level::Debug,
    };

    // Setup logging to stdout and stderr.
    logger::init(log_level);

    // Run vdot!
    if let Err(err) = vdot::run(args) {
        error!("{}", err);
        process::exit(1);
    }
}
