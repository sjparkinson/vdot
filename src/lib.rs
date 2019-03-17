//! This is documentation for the `vdot` crate.

use failure::Error;
use structopt::StructOpt;
use url::Url;

pub mod logger;

#[derive(StructOpt, Debug)]
#[structopt(author = "", about = "")]
pub struct Args {
    /// Vault paths.
    ///
    /// When using v1 of the Vault key value secrets engine your paths will look something like `secret/foo-bar`. If you are using v2 of the key value secrets engine, modify the path to look like `secret/data/foo-bar`.
    #[structopt(name = "PATH")]
    pub paths: Vec<String>,

    /// Your Vault token.
    #[structopt(long = "vault-token")]
    pub vault_token: Option<String>,

    /// The URL to access Vault.
    #[structopt(long = "vault-address")]
    pub vault_address: Option<Url>,

    /// Verbose mode (-v, -vv, or -vvv).
    /// 
    /// You can use `-v` to see the vdot debug messages. Use `-vv` to see vdot trace messages. Finally you can use `-vvv` to see all messages from all vdots dependencies too.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
}

/// Use the given command line arguments to run vdot.
///
/// # Examples
///
/// ```
/// use log::error;
/// use std::process;
/// use vdot::Args;
///
/// let args = Args {
///     paths: vec![],
///     vault_address: url::Url::parse("http://localhost:8200").unwrap(),
///     vault_token: "hunter2".to_string(),
///     verbose: 0,
/// };
///
/// if let Err(err) = vdot::run(args) {
///     error!("{}", err);
///     process::exit(1);
/// }
/// ```
///
/// # Errors
///
/// Returns an error if anything goes wrong.
pub fn run(_args: Args) -> Result<(), Error> {
    Ok(())
}
