use colored::*;
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub role: Vec<Role>,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Role {
    pub name: String,
    pub version: u8,
    pub prompt: String,
}

pub fn read_config() -> Result<Config, anyhow::Error> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    println!("Got API key {:?}", config.api_key);
    Ok(config)
}

impl Config {
    pub fn get_default_role(&self) -> Role {
        self.role[0].clone()
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone().unwrap_or_else(|| {
            env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
                println!(
                    "{}",
                    "No API key set, please set api_key in config or env variable OPENAI_API_KEY"
                        .red()
                );
                std::process::exit(-1);
            })
        })
    }
}
