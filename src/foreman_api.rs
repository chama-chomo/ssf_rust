use error_chain::error_chain;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Error as IoError;
use toml;

error_chain! {
foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
        MalformedToken(reqwest::header::ToStrError);
        Toml(toml::de::Error);
        Fs(IoError);
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Foreman {
    user: String,     // is taken from a config file
    password: String, // is taken from a config file
    url: String,
}

impl Foreman {
    // Creates Foreman session
    pub fn new(config_file: &str) -> Result<Foreman> {
        println!("Loading config...");
        let mut config = dirs::config_dir().expect("Error loading config dir.");
        config.push(config_file);
        let contents = fs::read_to_string(&config)?;

        let cfg_load: Foreman = toml::from_str(&contents)?;
        Ok(cfg_load)
    }

    pub fn get_machines_list(self) -> Result<Box<str>> {
        // Get the list of machines and their details by invoking GET to Foreman API.
        // auth
        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let url_endpoint = format!("{}/hosts", &self.url);
        let response = client
            .get(&url_endpoint)
            .basic_auth(&self.user, Some(&self.password))
            .send()?;

        // println!("Response: {:?}", results_get.text_with_charset("utf-8"));
        Ok(response
            .text_with_charset("utf-8")
            .unwrap()
            .into_boxed_str())
    }
}
