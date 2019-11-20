//! This is documentation for the `vdot` crate.

// #![deny(missing_docs)]

use failure::{Error, Fail};
use log::{debug, info, warn};
use reqwest::Url;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

pub mod logger;

#[derive(Fail, Debug)]
#[fail(display = "Vault responded with {} for the '{}' path", status, path)]
pub struct VaultResponseError {
    status: reqwest::StatusCode,
    path: String,
}

#[derive(StructOpt, Debug)]
#[structopt(usage = "vdot [FLAGS] [OPTIONS] <PATH>...")]
pub struct Args {
    /// Path to the Vault secrets
    ///
    /// If duplicate keys are found when providing more than one path the value from the first path will be saved.
    ///
    /// e.g. `vdot secret/foo secret/bar`
    ///
    /// See https://www.vaultproject.io/docs/secrets/kv/index.html for more information.
    #[structopt(name = "PATH", required = true)]
    pub paths: Vec<String>,

    /// Write to the given file
    ///
    /// e.g. `vdot --output .env.test secret/foo`
    #[structopt(
        short = "o",
        long = "output",
        default_value = ".env",
        value_name = "path",
        parse(from_os_str),
        display_order = 1
    )]
    pub output: PathBuf,

    /// Version of the key value secrets engine
    ///
    /// e.g. `vdot --kv 1 secret/foo`
    ///
    /// See https://www.vaultproject.io/docs/secrets/kv/index.html for more information.
    #[structopt(
        long = "kv",
        possible_values = &["1", "2"],
        value_name = "version",
        default_value = "2",
        display_order = 2
    )]
    pub vault_kv_version: u8,

    /// Vault token used to authenticate requests
    ///
    /// This can also be provided by setting the VAULT_TOKEN environment variable.
    ///
    /// e.g. `vdot --vault-token $(cat ~/.vault-token) secret/foo`
    ///
    /// See https://www.vaultproject.io/docs/concepts/auth.html#tokens for more information.
    #[structopt(
        long = "vault-token",
        env = "VAULT_TOKEN",
        value_name = "token",
        hide_env_values = true,
        display_order = 3
    )]
    pub vault_token: String,

    /// Vault server address
    ///
    /// This can also be provided by setting the VAULT_ADDR environment variable.
    ///
    /// e.g. `vdot --vault-address http://127.0.0.1:8200 secret/foo`
    #[structopt(
        long = "vault-address",
        env = "VAULT_ADDR",
        value_name = "address",
        display_order = 4
    )]
    pub vault_address: Url,

    /// Prints more verbose output
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

/// Use the given command line arguments to run vdot.
///
/// # Examples
///
/// ```
/// use log::error;
/// use std::path::PathBuf;
/// use std::process;
/// use vdot::Args;
/// use reqwest::Url;
///
/// let args = Args {
///     paths: vec![],
///     output: PathBuf::from(".env"),
///     vault_token: "hunter2".to_string(),
///     vault_address: Url::parse("http://127.0.0.1:8200").unwrap(),
///     vault_kv_version: 2,
///     verbose: false
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

    let mut paths = args.paths;

    // Reverse the order of paths so that latter paths with a duplicate variable name are overwritten.
    paths.reverse();

    for path in paths {
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
            }
            .into());
        }

        let resp: serde_json::Value = resp.json()?;
        let data = &resp["data"];

        // Handle the diffrent data formats for version 1 and 2 of the key-value secrets engine.
        let data = if data["metadata"]["version"].is_number() {
            data["data"].as_object().unwrap()
        } else {
            data.as_object().unwrap()
        };

        for (name, value) in data {
            let name = name.to_string();
            let value = match stringify_json_value(&value) {
                Some(value) => value,
                None => {
                    warn!(
                        "the value for {} in {} is an array or object and will be ignored",
                        name, path
                    );
                    continue;
                }
            };

            vars.insert(name, value);
        }
    }

    save_dotenv(args.output, vars)
}

fn save_dotenv(path: PathBuf, vars: HashMap<String, String>) -> Result<(), Error> {
    if Path::new(&path).is_file() {
        warn!("overwriting existing file at {}", path.display());
    }

    let file = File::create(&path)?;
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
        "saved {} environment {} to {}",
        count,
        if count == 1 { "variable" } else { "variables" },
        path.display()
    );

    Ok(())
}

fn stringify_json_value(value: &serde_json::Value) -> Option<String> {
    if value.is_string() {
        return Some(value.as_str().unwrap().to_string());
    }

    if value.is_boolean() {
        return Some(value.as_bool().unwrap().to_string());
    }

    if value.is_null() {
        return Some("".to_string());
    }

    if value.is_f64() {
        return Some(value.as_f64().unwrap().to_string());
    }

    if value.is_i64() {
        return Some(value.as_i64().unwrap().to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;

    #[test]
    fn stringify_json_value_converts_strings() {
        let json = json!("hello world");
        assert_eq!(stringify_json_value(&json), Some("hello world".to_string()));
    }

    #[test]
    fn stringify_json_value_converts_numbers() {
        let json = json!(42);
        assert_eq!(stringify_json_value(&json), Some("42".to_string()));

        let json = json!(-42);
        assert_eq!(stringify_json_value(&json), Some("-42".to_string()));

        let json = json!(4.2);
        assert_eq!(stringify_json_value(&json), Some("4.2".to_string()));
    }

    #[test]
    fn stringify_json_value_converts_null() {
        let json = json!(null);
        assert_eq!(stringify_json_value(&json["key"]), Some("".to_string()));
    }

    #[test]
    fn stringify_json_value_converts_booleans() {
        let json = json!(true);
        assert_eq!(stringify_json_value(&json), Some("true".to_string()));

        let json = json!(false);
        assert_eq!(stringify_json_value(&json), Some("false".to_string()));
    }

    #[test]
    fn stringify_json_value_does_not_convert_arrays_and_objects() {
        let json = json!({});
        assert_eq!(stringify_json_value(&json), None);

        let json = json!([]);
        assert_eq!(stringify_json_value(&json), None);
    }
}
