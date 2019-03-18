//! This is documentation for the `vdot` crate.

use directories::UserDirs;
use failure::{Error, ResultExt};
use log::debug;
use std::{env, fs};
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
    #[structopt(name = "PATH")]
    pub paths: Vec<String>,

    /// Your Vault token.
    #[structopt(long = "vault-token")]
    pub vault_token: Option<String>,

    /// The URL to access Vault.
    #[structopt(long = "vault-address")]
    pub vault_address: Option<Url>,

    /// Verbose mode (-v, or -vv).
    ///
    /// You can use `-v` to see debug messages. Use `-vv` to see trace messages.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
}

impl Args {
    /// Build an instance of [`Args`] from the environment.
    ///
    /// This will populate `vault_token` from the contents of `~/.vault-token`, and `vault_address` from the value of the `VAULT_ADDR` environment variable.
    ///
    /// Both `~/.vault-token` and the `VAULT_ADDR` environment variable are supported by the official Vault command line tool.
    ///
    /// [`Arg`]: ../../vdot/struct.Args.html
    ///
    /// # Examples
    ///
    /// ```
    /// let args = vdot::Args::default();
    /// 
    /// vdot::Args::from_env(args);
    /// ```
    ///
    /// # Errors
    ///
    /// Will throw an [`io::Error`] if `~/.vault-token` doesn't exist.
    ///
    /// Will throw a [`ParseError`] if the `VAULT_ADDR` environment variable cannot parse into a [`Url`].
    ///
    /// [`ParseError`]: ../../url/enum.ParseError.html
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    /// [`Url`]: ../../url/struct.Url.html
    pub fn from_env(args: Args) -> Result<Args, Error> {
        let mut args = args.clone();

        // Find the Vault token by reading `~/.vault-token`.
        if args.vault_token == None {
            debug!("no vault token command line option, reading it from ~/.vault-token");

            let token_path = UserDirs::new().unwrap().home_dir().join(".vault-token");
            let vault_token = fs::read_to_string(token_path)
                .context("couldn't read a token from ~/.vault-token, try running `vault login`")?;

            args.vault_token = Some(vault_token);
        }

        // Read the Vault address from the VAULT_ADDR environment variable.
        if args.vault_address == None {
            debug!("no vault address command line option, reading it from the VAULT_ADDR environment variable");

            let vault_address = env::var("VAULT_ADDR").context("the VAULT_ADDR environment variable must be set, e.g. `export VAULT_ADDR=http://127.0.0.1:8200`")?;
            let vault_address = vault_address.trim_end_matches('/');
            let vault_address = Url::parse(vault_address)
                .context(format!("{} is not a valid Vault address", vault_address))?;

            args.vault_address = Some(vault_address);
        }

        Ok(args)
    }

    /// Checks if `vault_token` and `vault_address` both have a value.bool
    /// 
    /// # Examples
    /// 
    /// ```
    /// vdot::Args::default().is_valid();
    /// ```
    pub fn is_valid(&self) -> bool {
        if self.vault_token != None && self.vault_address != None {
            return true;
        }

        false
    }
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
