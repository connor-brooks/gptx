use anyhow::{Error, Result};
use serde::Deserialize;
use std::{collections::HashMap, env, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub role: HashMap<String, Role>,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Role {
    pub version: u8,
    pub prompt: String,
}

pub fn read_config() -> Result<Config, Error> {
    let home_dir = env::var("HOME")?;
    let config_str = fs::read_to_string(home_dir + "/.config/gptx/config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

impl Config {
    pub fn get_role(&self, role: String) -> Role {
        if let Some(v) = self.role.get(&role) {
            v.clone()
        } else {
            crate::print_fatal!("Could not find role: ".red(), role);
        }
    }

    pub fn get_api_key(&self) -> String {
        if let Some(k) = &self.api_key {
            k.to_string()
        } else if let Ok(k) = env::var("OPENAI_API_KEY") {
            k
        } else {
            crate::print_fatal!(
                "No API key set:".red(),
                "please set api_key in config or env variable OPENAI_API_KEY"
            );
        }
    }
}
