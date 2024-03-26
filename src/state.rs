use crate::cli;
use crate::config;
use atty::Stream;
use chatgpt::prelude::*;
use colored::*;

pub struct TgptState {
    pub inital_message: Option<String>,
    pub role: String,
    pub repl_mode: bool,
    pub piped: bool,
    pub model_4: bool,
    pub verbose: bool,
    pub conversation: Option<Conversation>,
    pub api_key: String,
    pub args: cli::Args,
    pub config: config::Config,
}

fn print_state(s: &TgptState) {
    println!("{}", "+-----------------------------+".yellow());
    println!("{}", "TGPT State:".yellow());
    println!("{}", "+-----------------------------+".yellow());
    println!("Initial message:      {:?}", s.inital_message);
    println!("Role:                 {:?}", s.role);
    println!("REPL mode:            {:?}", s.repl_mode);
    println!("Piped content:        {:?}", s.piped);
    println!("GPT4 enabled:         {:?}", s.model_4);
    println!("Verbose:              {:?}", s.verbose);
    println!("{}", "+-----------------------------+".yellow());
}

pub fn init(conf: config::Config, args: cli::Args) -> TgptState {
    let mut state = TgptState {
        inital_message: args.prompt.clone(),
        role: args.role.clone(),
        repl_mode: args.repl_mode || args.prompt.is_none(),
        piped: !atty::is(Stream::Stdin),
        model_4: args.model_4,
        verbose: args.verbose,
        conversation: None,
        api_key: conf.get_api_key(),
        args,
        config: conf,
    };

    // GPT4 or 3:
    let gpt_client = if state.model_4 {
        ChatGPT::new_with_config(
            &state.api_key,
            ModelConfigurationBuilder::default()
                .engine(ChatGPTEngine::Gpt4)
                .build()
                .unwrap(),
        )
        .unwrap()
    } else {
        ChatGPT::new(&state.api_key).unwrap()
    };

    let conversation =
        gpt_client.new_conversation_directed(state.config.get_role(state.role.clone()).prompt);
    state.conversation = Some(conversation);

    if state.verbose {
        print_state(&state);
    }

    state
}
