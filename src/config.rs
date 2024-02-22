use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub role: Vec<Role>,
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub name: String,
    pub version: u8,
    pub prompt: String,
}

pub fn read_config() -> Result<Config, anyhow::Error> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
