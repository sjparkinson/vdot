#[macro_use]
extern crate failure;

use directories::UserDirs;
use docopt::ArgvMap;
use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::{env, fs};
use url::Url;

use failure::Error;

#[derive(Debug)]
pub struct Config {
    pub paths: Vec<String>,
    pub token: String,
    pub address: String,
}

impl Config {
    pub fn new(args: &ArgvMap) -> Result<Self, Error> {
        let token_path = UserDirs::new().unwrap().home_dir().join(".vault-token");

        let token = if let Ok(token) = fs::read_to_string(token_path) {
            String::from(token.trim())
        } else {
            return Err(format_err!(
                "~/.vault-token must exist, try running `vault login`"
            ));
        };

        let address = match env::var("VAULT_ADDR") {
            Ok(addr) => addr,
            Err(_) => {
                return Err(format_err!("the $VAULT_ADDR environment variable must be set, e.g. `export VAULT_ADDR=https://vault.example.com`"))
            }
        };

        let paths = args.get_vec("<path>");
        let paths = paths.into_iter().map(String::from).collect();

        Ok(Self {
            paths,
            token,
            address,
        })
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    let http = reqwest::Client::new();

    let mut vars = EnvironmentVariables::new();

    for path in config.paths {
        let url = format_vault_url(config.address.as_str(), path.as_str())?;

        let req = http
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", config.token));

        let mut resp: Response = req.send()?;

        if !resp.status().is_success() {
            return Err(format_err!(
                "vault responded with a {} status code for the '{}' path",
                resp.status().as_str(),
                path.clone()
            ));
        }

        let resp: Value = resp.json()?;

        let data = resp["data"].as_object().unwrap();

        for (name, value) in data {
            vars.insert(name.to_string(), value.to_string());
        }
    }

    let file = File::create(".env")?;
    let mut buf = BufWriter::new(file);

    let count = vars.len();

    save_environment_variables(vars, &mut buf)?;

    println!("Saved {} environment variables to .env", count);

    Ok(())
}

type EnvironmentVariables = HashMap<String, String>;

fn save_environment_variables(variables: EnvironmentVariables, w: &mut Write) -> Result<(), Error> {
    for (variable, value) in variables {
        writeln!(w, "{}={}", variable, value)?;
    }

    Ok(())
}

fn format_vault_url(address: &str, path: &str) -> Result<Url, Error> {
    let url = Url::parse(address)?;
    let url = url.join("v1/")?;
    let url = url.join(path)?;

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_environment_variables_formats_for_dotenv() {
        let mut vars = EnvironmentVariables::new();
        vars.insert(String::from("foo"), String::from("bar"));

        let mut dotenv = Vec::new();

        save_environment_variables(vars, &mut dotenv).unwrap();

        assert_eq!(dotenv, b"foo=bar\n")
    }

    #[test]
    fn test_format_vault_url_formats_to_v1_api() {
        assert_eq!(
            format_vault_url("https://vault.example.com", "secret/foo-bar").unwrap(),
            Url::parse("https://vault.example.com/v1/secret/foo-bar").unwrap()
        );
    }

    #[test]
    fn test_format_vault_url_errors_on_bad_address() {
        assert!(format_vault_url("foo-bar", "fizz-buzz").is_err());
    }
}
