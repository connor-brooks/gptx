use crate::chat;
use crate::state;
use anyhow::{anyhow, Error, Result};
use clap::Parser;
use colored::*;
use rustyline::{
    config::Configurer, error::ReadlineError, Cmd, DefaultEditor, EditMode, EventHandler, KeyEvent,
};

/// TGPT is a terminal helper program based on OpenAI's GPT3 and GPT4 APIs... WIP :)
/// Opening TGPT with no arguments will launch REPL mode, unless you have specified
/// an inital prompt.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Role to use
    #[clap(short = 'r', long = "role", default_value = "default")]
    pub role: String,

    /// Force ChatGPT 4 model
    #[clap(short = '4', long = "model4")]
    pub model_4: bool,

    /// Activate REPL mode
    #[arg(short = 'R', long = "repl")]
    pub repl_mode: bool,

    /// Verbose output
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Prompt
    #[clap()]
    pub prompt: Option<String>,
}

pub async fn repl_loop(state: &mut state::TgptState) -> Result<(), Error> {
    let mut rl = DefaultEditor::new()?;
    rl.set_edit_mode(EditMode::Vi);
    rl.bind_sequence(KeyEvent::alt('\x0d'), EventHandler::Simple(Cmd::Newline));
    // Sadly this does not work (triggers a newline) :(
    // rl.bind_sequence(
    //     KeyEvent::new('\x0d', Modifiers::SHIFT),
    //     EventHandler::Simple(Cmd::Newline),
    // );
    let prompt_str = ">> ".bold();
    loop {
        let readline = rl.readline(&prompt_str);
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                chat::process_single_msg(state, line)
                    .await
                    .unwrap_or_else(|e| {
                        println!("{} {}", "an error occured:".red(), e);
                    });
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break Ok(());
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break Ok(());
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break Err(anyhow!("oops"));
            }
        }
        if !state.repl_mode {
            break Ok(());
        }
    }
}
