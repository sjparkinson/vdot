//! This is documentation for the `vdot` crate.

use failure::{Error, Fail};
use log::{debug, info, warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use structopt::StructOpt;
use url::Url;

pub mod logger;

#[derive(Fail, Debug)]
#[fail(display = "Vault responded with {} for the '{}' path", status, path)]
pub struct VaultResponseError {
    status: reqwest::StatusCode,
    path: String,
}

#[derive(StructOpt, Debug)]
#[structopt(author = "", about = "", usage = "vdot [FLAGS] <OPTIONS> <PATH>...")]
pub struct Args {
    /// Your Vault paths.
    ///
    /// Something like `secret/foo-bar` for v1, and `secret/data/foo-bar` for v2 of the Vault key-value secrets engine.
    ///
    /// See https://www.vaultproject.io/docs/secrets/kv/index.html for more information.
    #[structopt(name = "PATH", raw(required = "true"))]
    pub paths: Vec<String>,

    /// Your Vault token.
    ///
    /// This can be provided by setting the VAULT_TOKEN environment variable.
    #[structopt(long = "vault-token", env = "VAULT_TOKEN", hide_env_values = true)]
    pub vault_token: String,

    /// The URL to access Vault.
    ///     
    /// This can be provided by setting the VAULT_ADDR environment variable.
    #[structopt(long = "vault-address", env = "VAULT_ADDR", hide_env_values = true)]
    pub vault_address: Url,

    /// Verbose mode.
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
///     vault_token: "hunter2".to_string(),
///     vault_address: url::Url::parse("http://127.0.0.1:8200").unwrap(),
///     verbose: 0
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
/// Returns an error if anything goes wrong, and exits the process with a status code of 1.
pub fn run(args: Args) -> Result<(), Error> {
    // Create a new http client to make use of connection pooling.
    let http = reqwest::Client::new();

    // Key-value store for the environment variable downloaded from Vault.
    let mut vars: HashMap<String, String> = HashMap::new();

    for path in args.paths {
        // Build the Vault API url.
        let url = args.vault_address.join("v1/")?;
        let url = url.join(path.as_str())?;

        debug!("making request to \"{}\"", url);

        let req = http.get(url).header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", args.vault_token),
        );

        let mut resp = req.send()?;

        if !resp.status().is_success() {
            return Err(VaultResponseError {
                status: resp.status(),
                path,
            })?;
        }

        let resp: serde_json::Value = resp.json()?;
        let data = &resp["data"];

        // Handle the diffrent data formats for version 1 and 2 of the key-value secrets engine.
        if data["metadata"]["version"].is_number() {
            for (name, value) in data["data"].as_object().unwrap() {
                vars.insert(name.to_string(), value.as_str().unwrap().to_string());
            }
        } else {
            for (name, value) in data.as_object().unwrap() {
                vars.insert(name.to_string(), value.as_str().unwrap().to_string());
            }
        }
    }

    if Path::new(".env").is_file() {
        warn!("overwriting existing .env file");
    }

    let file = File::create(".env")?;
    let mut buf = BufWriter::new(file);

    let count = vars.len();

    for (variable, value) in vars {
        if value.contains('\n') {
            let value = value.replace("\n", "\\n");
            writeln!(buf, "{}=\"{}\"", variable, value)?;
        } else {
            writeln!(buf, "{}={}", variable, value)?;
        }
    }

    info!(
        "saved {} environment {} to .env",
        count,
        if count == 1 { "variable" } else { "variables" }
    );

    Ok(())
}
