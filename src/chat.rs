use crate::cli;
use crate::state;
use anyhow::{Error, Result};
use chatgpt::prelude::*;
use colored::*;
use futures_util::StreamExt;

pub async fn process_piped_msg(state: &mut state::GptxState) -> Result<()> {
    let piped_msg = cli::read_pipe();
    let inital_msg = state.inital_message.clone().unwrap_or("".to_string());
    let final_msg = format!("{}\n{}", inital_msg, piped_msg);

    cli::print_verbose(
        &format!("{} {}", "built pipe input:".bold(), final_msg),
        state.verbose,
    );

    if let Err(e) = process_single_msg(state, final_msg).await {
        println!(
            "{}. {}",
            "An error occured processing your message: ".red(),
            e
        );
    }

    if state.repl_mode {
        println!(
            "{} REPL mode not avaiable when data is piped into GPTX!",
            String::from("Note:").red()
        );
    }
    std::process::exit(0);
}

pub async fn process_single_msg(
    state: &mut state::GptxState,
    message: String,
) -> Result<(), Error> {
    let mut stream = match &mut state.conversation {
        Some(conv) => conv.send_message_streaming(message).await?,
        None => std::process::exit(-1),
    };

    let mut output: Vec<ResponseChunk> = Vec::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            ResponseChunk::Content {
                delta,
                response_index,
            } => {
                cli::print_resp_word(&delta, &state.resp_color);
                output.push(ResponseChunk::Content {
                    delta,
                    response_index,
                });
            }
            other => {
                output.push(other);
            }
        }
    }

    let messages = ChatMessage::from_response_chunks(output);
    match &mut state.conversation {
        Some(conv) => conv.history.push(messages[0].to_owned()),
        None => std::process::exit(-1),
    };

    println!(); // for rustyline
    Ok(())
}
