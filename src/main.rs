use anyhow::Result;
use colored::*;
use format::print_verbose;
use std::io;
mod chat;
mod format;
mod repl;
mod role;
mod state;

async fn handle_pipe(state: &mut state::TgptState) {
    let mut piped = String::new();
    match io::stdin().read_line(&mut piped) {
        Ok(_) => {}
        Err(error) => {
            println!("Error getting piped: {:?}", error);
        }
    }

    let prompt = state.inital_message.clone().unwrap_or("".to_string());
    let final_str = format!("{}\n{}", prompt, piped);

    print_verbose(
        &format!("{} {}", "built pipe input:".bold(), final_str),
        state.verbose,
    );

    let _ = chat::send_dialog(state, final_str).await;

    if state.args.repl_mode {
        println!(
            "{} REPL mode not avaiable when data is piped into TGPT!",
            String::from("Note:").red()
        );
    }
    std::process::exit(0);
}

/// Requires the `streams` crate feature
#[tokio::main]
async fn main() -> Result<()> {
    let mut state = state::init();

    if state.piped {
        format::print_verbose("handling piped data", state.verbose);
        handle_pipe(&mut state).await;
    }

    if let Some(s) = state.inital_message.clone() {
        let _ = chat::send_dialog(&mut state, s).await;

        if !state.repl_mode {
            format::print_verbose("no REPL mode, exiting", state.verbose);
            return Ok(());
        }
    }

    repl::repl_loop(&mut state).await
}
