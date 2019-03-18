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

    // We have a valid Args instance, we can run.
    if args.is_valid() {
        return run(args);
    }

    // We don't have the Vault configuration from the command line,
    // try and get it from the environment.
    let args = Args::from_env(args).unwrap_or_else(|err| {
        error!("{}", err);
        process::exit(exitcode::USAGE);
    });

    run(args)
}

fn run(args: Args) {
    if let Err(err) = vdot::run(args) {
        error!("{}", err);
        process::exit(1);
    }
}