use failure::Error;
use structopt::StructOpt;
use url::Url;

pub mod log;

#[derive(StructOpt, Debug)]
pub struct Args {
    /// Vault paths.
    #[structopt(name = "PATH")]
    pub paths: Vec<String>,

    /// Your Vault token.
    #[structopt(long = "vault-token")]
    pub vault_token: String,

    /// The URL to access Vault.
    #[structopt(long = "vault-address")]
    pub vault_address: Url,

    /// Verbose mode (-v, -vv, or -vvv).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
}

pub fn run() -> Result<(), Error> {
    Ok(())
}
