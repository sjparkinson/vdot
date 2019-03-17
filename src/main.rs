use log::error;
use std::process;
use structopt::StructOpt;
use vdot::Args;

fn main() {
    vdot::log::init(log::Level::Info);

    let args = Args::from_args();

    println!("{:#?}", args);

    if let Err(err) = vdot::run() {
        error!("{}", err);
        process::exit(1);
    }
}
