use colored::*;
use serde::Deserialize;
use std::{collections::HashMap, env, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub role: HashMap<String, Role>,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Role {
    pub name: String,
    pub version: u8,
    pub prompt: String,
}

pub fn read_config() -> Result<Config, anyhow::Error> {
    let home_dir = env::var("HOME")?;
    let config_str = fs::read_to_string(home_dir + "/.config/tgpt/config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    for (name, role) in config.role.clone() {
        println!("{} {}", name, role.prompt)
    }
    Ok(config)
}

impl Config {
    pub fn get_role(&self, role: String) -> Role {
        self.role[&role].clone()
    }

    pub fn get_api_key(&self) -> String {
        if self.api_key.is_some() {
            self.api_key.clone().unwrap()
        } else if env::var("OPENAI_API_KEY").is_ok() {
            env::var("OPENAI_API_KEY").unwrap()
        } else {
            println!(
                "{}",
                "No API key set, please set api_key in config or env variable OPENAI_API_KEY".red()
            );
            std::process::exit(-1);
        }
    }
}
