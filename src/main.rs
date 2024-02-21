use anyhow::Result;
mod chat;
mod format;
mod repl;
mod role;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let mut state = state::init();

    if state.piped {
        format::print_verbose("handling piped data", state.verbose);
        chat::handle_pipe(&mut state).await;
    }

    if let Some(s) = state.inital_message.clone() {
        format::print_verbose("handling argument message", state.verbose);
        let _ = chat::send_dialog(&mut state, s).await;

        if !state.repl_mode {
            format::print_verbose("no REPL mode, exiting", state.verbose);
            return Ok(());
        }
    }

    format::print_verbose("entering repl", state.verbose);
    repl::repl_loop(&mut state).await
}
