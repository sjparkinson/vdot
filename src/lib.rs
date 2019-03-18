//! This is documentation for the `vdot` crate.

use failure::Error;
use structopt::StructOpt;
use url::Url;

pub mod logger;

#[derive(StructOpt, Debug, Clone, Default)]
#[structopt(author = "", about = "")]
pub struct Args {
    /// Vault paths.
    ///
    /// When using v1 of the Vault key value secrets engine your paths will look something like
    /// `secret/foo-bar`. If you are using v2 of the key value secrets engine, modify the path to
    /// look like `secret/data/foo-bar`.
    #[structopt(name = "PATH", raw(required = "true"))]
    pub paths: Vec<String>,

    /// Your Vault token.
    #[structopt(long = "vault-token", env = "VAULT_TOKEN", hide_env_values = true)]
    pub vault_token: Option<String>,

    /// The URL to access Vault.
    #[structopt(long = "vault-address", env = "VAULT_ADDR", hide_env_values = true)]
    pub vault_address: Option<Url>,

    /// Verbose mode (-v, or -vv).
    ///
    /// You can use `-v` to see debug messages. Use `-vv` to see trace messages.
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
/// if let Err(err) = vdot::run(Args::default()) {
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
