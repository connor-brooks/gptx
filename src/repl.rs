use crate::chat;
use crate::state;
use anyhow::{anyhow, Error, Result};
use colored::*;
use rustyline::{
    config::Configurer, error::ReadlineError, Cmd, DefaultEditor, EditMode, EventHandler, KeyEvent,
};

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
