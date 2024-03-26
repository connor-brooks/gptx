use anyhow::Result;
use clap::Parser;
mod chat;
mod cli;
mod config;
mod format;
mod repl;
mod state;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = config::read_config().unwrap_or_else(|e| {
        println!("{} {}", "config could not be read, exiting:".red(), e);
        std::process::exit(-1)
    });

    let args = cli::Args::parse();

    let mut state = state::init(conf, args);

    if state.piped {
        format::print_verbose("handling piped data", state.verbose);
        chat::process_piped_msg(&mut state).await?;
    }

    if let Some(s) = state.inital_message.clone() {
        format::print_verbose("handling argument message", state.verbose);
        let _ = chat::process_single_msg(&mut state, s).await;

        if !state.repl_mode {
            format::print_verbose("no REPL mode, exiting", state.verbose);
            return Ok(());
        }
    }

    format::print_verbose("entering repl", state.verbose);
    repl::repl_loop(&mut state).await
}
