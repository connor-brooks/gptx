use crate::state::TgptState;
use colored::*;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    role: Vec<Role>,
}

#[derive(Debug, Deserialize)]
struct Role {
    name: String,
    version: u8,
    prompt: String,
}

fn read_config() -> Result<Config, anyhow::Error> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

//TODO cleanup role builder
pub fn role_builder(_state: &TgptState) -> String {
    let config = read_config().unwrap_or_else(|e| {
        println!("{}", "config could not be read, exiting:".red());
        std::process::exit(-1)
    });

    config.role[0].prompt.clone()

    //String::from("IMPORTANT: if asked a technical question only only provide code as output without any description. Provide only code in plain text format without Markdown formatting. Do not include symbols such as ``` or ```python")
}
