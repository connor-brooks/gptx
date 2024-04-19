use anyhow::Result;
use clap::Parser;
mod chat;
mod cli;
mod config;
mod state;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = config::read_config().unwrap_or_else(|e| {
        eprintln!("{} {}", "Config could not be read, exiting:".red(), e);
        std::process::exit(-1)
    });

    let args = cli::Args::parse();
    let mut state = state::init(conf, args).unwrap_or_else(|e| {
        eprintln!("{} {}", "Problem starting up:".red(), e);
        std::process::exit(-1)
    });

    if state.piped {
        cli::print_verbose("Handling piped data", state.verbose);
        chat::process_piped_msg(&mut state).await?;
    }

    if state.inital_message.is_some() {
        cli::print_verbose("Handling argument message", state.verbose);
        let initial_msg = state.inital_message.clone().unwrap();
        chat::process_single_msg(&mut state, initial_msg)
            .await
            .unwrap_or_else(|e| {
                eprintln!("{} {}", "Problem connecting to API:".red(), e);
                std::process::exit(-1)
            });

        if !state.repl_mode {
            cli::print_verbose("No REPL mode, exiting", state.verbose);
            return Ok(());
        }
    }

    cli::print_verbose("Entering repl", state.verbose);
    cli::repl_loop(&mut state).await
}
