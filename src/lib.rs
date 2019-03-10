#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

use directories::UserDirs;
use docopt::ArgvMap;
use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
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
            Ok(addr) => addr.trim_end_matches('/').to_string(),
            Err(_) => {
                return Err(format_err!("the $VAULT_ADDR environment variable must be set, e.g. `export VAULT_ADDR=https://vault.example.com`"))
            }
        };

        let paths = args.get_vec("<path>");
        let paths: Vec<String> = paths.into_iter().map(String::from).collect();

        Ok(Self {
            paths,
            token,
            address,
        })
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    // Create a new http client to make use of connec
    let http = reqwest::Client::new();

    let mut vars: HashMap<String, String> = HashMap::new();

    for path in config.paths {
        let url = format_vault_url(config.address.as_str(), path.as_str())?;

        debug!("making request to \"{}\"", url);

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
        let data = data
            .into_iter()
            .map(|(name, value)| (name.to_string(), String::from(value.as_str().unwrap())));

        for (name, value) in data {
            vars.insert(name, value);
        }
    }

    if Path::new(".env").is_file() {
        warn!("overwriting existing .env file");
    }

    let file = File::create(".env")?;
    let mut buf = BufWriter::new(file);

    let count = vars.len();

    save_environment_variables(vars, &mut buf)?;

    info!(
        "saved {} environment {} to .env",
        count,
        if count == 1 { "variable" } else { "variables" }
    );

    Ok(())
}

fn save_environment_variables<I>(variables: I, w: &mut Write) -> Result<(), Error>
where
    I: IntoIterator<Item = (String, String)>,
{
    for (variable, value) in variables {
        if value.contains("\n") {
            let value = value.replace("\n", "\\n");
            writeln!(w, "{}=\"{}\"", variable, value)?;
        } else {
            writeln!(w, "{}={}", variable, value)?;
        }
    }

    Ok(())
}

fn format_vault_url(address: &str, path: &str) -> Result<Url, Error> {
    let url = Url::parse(address)?;

    if (url.scheme() != "http" && url.scheme() != "https") || !url.has_authority() {
        return Err(format_err!(
            "only http and https schemes are allowed in VAULT_ADDR"
        ));
    }

    let url = url.join("v1/")?;
    let url = url.join(path)?;

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_save_environment_variables_formats_for_dotenv() {
        // Using BTreeMap to get a consistent order.
        let mut vars: BTreeMap<String, String> = BTreeMap::new();
        vars.insert(String::from("fizz"), String::from("buzz"));
        vars.insert(String::from("foo"), String::from("bar"));

        let mut dotenv = Vec::new();

        save_environment_variables(vars, &mut dotenv).unwrap();

        assert_eq!(
            String::from_utf8(dotenv).unwrap(),
            String::from("fizz=buzz\nfoo=bar\n")
        );
    }

    #[test]
    fn test_save_environment_variables_quotes_and_escapes_multi_line_values() {
        // Using BTreeMap to get a consistent order.
        let mut vars: BTreeMap<String, String> = BTreeMap::new();
        vars.insert(
            String::from("EXAMPLE"),
            String::from("this is a\nmulti-line\nvalue"),
        );

        let mut dotenv = Vec::new();

        save_environment_variables(vars, &mut dotenv).unwrap();

        assert_eq!(
            String::from_utf8(dotenv).unwrap(),
            String::from("EXAMPLE=\"this is a\\nmulti-line\\nvalue\"\n")
        );
    }

    #[test]
    fn test_format_vault_url_formats_to_v1_api() {
        assert_eq!(
            format_vault_url("http://vault.example.com", "secret/foo-bar").unwrap(),
            Url::parse("http://vault.example.com/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("http://vault.example.com", "secret/foo-bar").unwrap(),
            Url::parse("http://vault.example.com/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("https://vault.example.com", "secret/foo-bar").unwrap(),
            Url::parse("https://vault.example.com/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("https://vault.example.com/", "secret/foo-bar").unwrap(),
            Url::parse("https://vault.example.com/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("http://127.0.0.1", "secret/foo-bar").unwrap(),
            Url::parse("http://127.0.0.1/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("http://127.0.0.1/", "secret/foo-bar").unwrap(),
            Url::parse("http://127.0.0.1/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("https://127.0.0.1", "secret/foo-bar").unwrap(),
            Url::parse("https://127.0.0.1/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("https://127.0.0.1/", "secret/foo-bar").unwrap(),
            Url::parse("https://127.0.0.1/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("http://[::1]", "secret/foo-bar").unwrap(),
            Url::parse("http://[::1]/v1/secret/foo-bar").unwrap()
        );

        assert_eq!(
            format_vault_url("http://[::1]/", "secret/foo-bar").unwrap(),
            Url::parse("http://[::1]/v1/secret/foo-bar").unwrap()
        );
    }

    #[test]
    fn test_format_vault_url_errors_on_bad_address() {
        assert!(format_vault_url("host-with-no-scheme", "secret/fizz-buzz").is_err());
        assert!(format_vault_url("https://", "secret/fizz-buzz").is_err());
        assert!(format_vault_url("http//localhost", "secret/fizz-buzz").is_err());

        // Only accept http or https.
        assert!(format_vault_url("data:text/plain", "secret/fizz-buzz").is_err());
        assert!(format_vault_url("ftp://localhost", "secret/fizz-buzz").is_err());
        assert!(format_vault_url("unix://localhost", "secret/fizz-buzz").is_err());
    }
}
