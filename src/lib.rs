use docopt::ArgvMap;
use std::boxed::Box;
use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub paths: Vec<String>,
    pub token: String,
    pub address: String,
}

impl Config {
    pub fn new(args: &ArgvMap) -> Result<Config, Box<dyn Error>> {
        // Read Vault token from filesystem, or error with prompt to login.
        let home = env::var("HOME")?;
        let token = fs::read_to_string(format!("{}/.vault-token", home))?;

        // Read Vault address from environment variable.
        let address = env::var("VAULT_ADDR")?;

        let paths = args.get_vec("<path>");
        let paths = paths.into_iter().map(|s| s.to_string()).collect();

        Ok(Config {
            paths,
            token,
            address,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Hello world!");

    println!("{:?}", config);

    Ok(())
}
