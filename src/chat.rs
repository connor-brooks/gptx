use crate::state;
use anyhow::{Error, Result};
use chatgpt::prelude::*;
use colored::*;
use futures_util::StreamExt;
use std::io::{stdout, Write};

pub async fn send_dialog(state: &mut state::TgptState, message: String) -> Result<(), Error> {
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
                print!("{}", delta.yellow());
                stdout().lock().flush().unwrap();
                output.push(ResponseChunk::Content {
                    delta,
                    response_index,
                });
            }
            other => output.push(other),
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
