use crate::cli;
use crate::config;
use anyhow::{Error, Result};
use atty::Stream;
use chatgpt::prelude::*;
use colored::*;

pub struct GptxState {
    pub inital_message: Option<String>,
    pub role: String,
    pub repl_mode: bool,
    pub piped: bool,
    pub model_4: bool,
    pub verbose: bool,
    pub conversation: Option<Conversation>,
}

fn print_state(s: &GptxState) {
    println!("{}", "+-----------------------------+".yellow());
    println!("{}", "GPTX State:".yellow());
    println!("{}", "+-----------------------------+".yellow());
    println!("Initial message:      {:?}", s.inital_message);
    println!("Role:                 {:?}", s.role);
    println!("REPL mode:            {:?}", s.repl_mode);
    println!("Piped content:        {:?}", s.piped);
    println!("GPT4 enabled:         {:?}", s.model_4);
    println!("Verbose:              {:?}", s.verbose);
    println!("{}", "+-----------------------------+".yellow());
}

pub fn init(conf: config::Config, args: cli::Args) -> Result<GptxState, Error> {
    let mut state = GptxState {
        inital_message: args.prompt.clone(),
        role: args.role.clone(),
        repl_mode: args.repl_mode || args.prompt.is_none(),
        piped: !atty::is(Stream::Stdin),
        model_4: args.model_4,
        verbose: args.verbose,
        conversation: None,
    };

    if state.piped {
        state.repl_mode = false
    }

    let role = conf.get_role(state.role.clone());
    state.model_4 |= role.version == 4;

    // GPT4 or 3:
    let gpt_client = if state.model_4 {
        ChatGPT::new_with_config(
            conf.get_api_key(),
            ModelConfigurationBuilder::default()
                .engine(ChatGPTEngine::Gpt4)
                .build()?,
        )?
    } else {
        ChatGPT::new(conf.get_api_key())?
    };

    let conversation = gpt_client.new_conversation_directed(role.prompt);
    state.conversation = Some(conversation);

    if state.verbose {
        print_state(&state);
    }

    Ok(state)
}
