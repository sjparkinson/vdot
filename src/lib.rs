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
    pub fn new(args: &ArgvMap) -> Result<Config, Error> {
        let token_path = UserDirs::new().unwrap().home_dir().join(".vault-token");
        let token = match fs::read_to_string(token_path) {
            Ok(token) => String::from(token.trim()),
            Err(_) => {
                return Err(format_err!(
                    "~/.vault-token must exist, try running `vault login`"
                ));
            }
        };

        let address = match env::var("VAULT_ADDR") {
            Ok(addr) => addr,
            Err(_) => {
                return Err(format_err!("the $VAULT_ADDR environment variable must be set, e.g. `export VAULT_ADDR=https://vault.example.com`"))
            }
        };

        let paths = args.get_vec("<path>");
        let paths = paths.into_iter().map(String::from).collect();

        Ok(Config {
            paths,
            token,
            address,
        })
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    let http = reqwest::Client::new();

    let url = Url::parse(config.address.as_str())?;
    let url = url.join("v1/")?;

    let mut vars: HashMap<String, String> = HashMap::new();

    for path in config.paths {
        let req = http
            .get(url.join(&path)?)
            .header(AUTHORIZATION, format!("Bearer {}", config.token));

        let mut resp: Response = req.send()?;

        if !resp.status().is_success() {
            let status = resp.status();
            let status = status.as_str();

            return Err(format_err!(
                "vault responded with a {} status code for the '{}' path",
                status,
                path
            ));
        }

        let resp: Value = resp.json()?;

        let data = resp["data"].as_object().unwrap();

        for (name, value) in data {
            vars.insert(name.to_string(), value.to_string());
        }
    }

    let mut buf = BufWriter::new(File::create(".env")?);

    let count = vars.len();

    for (name, value) in vars {
        writeln!(&mut buf, "{}={}", name, value)?;
    }

    println!("Saved {} environment variables to .env", count);

    Ok(())
}
