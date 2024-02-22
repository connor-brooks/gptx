use std::env;

use crate::config;
use crate::format::print_verbose;
use crate::role;
use atty::Stream;
use chatgpt::prelude::*;
use clap::Parser;
use colored::*;

pub struct TgptState {
    pub inital_message: Option<String>,
    pub repl_mode: bool,
    pub piped: bool,
    pub gen_cmd: bool,
    pub model_4: bool,

    pub verbose: bool,

    pub attach_path: Option<String>,
    pub attach_content: Option<Vec<u8>>,

    pub conversation: Option<Conversation>,
    pub api_key: Option<String>,

    pub args: Args,

    pub config: config::Config,
}

/// TGPT is a terminal helper program based on OpenAI's GPT3 and GPT4 APIs... WIP :)
/// Opening TGPT with no arguments will launch REPL mode, unless you have specified
/// an inital prompt.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Activate REPL mode
    #[arg(short = 'r', long = "repl")]
    pub repl_mode: bool,

    /// Activate echo
    #[clap(short = 'e', long = "echo")]
    echo: bool,

    /// Generate shell commands
    #[clap(short = 'c', long = "command")]
    command: bool,

    /// Enable ChatGPT 4 model
    #[clap(short = '4', long = "model4")]
    model_4: bool,

    /// Attach a file
    #[clap(short = 'a', long = "attach")]
    attachment: Option<String>,

    /// Verbose output
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    /// Prompt
    #[clap()]
    prompt: Option<String>,
}

fn print_state(s: &TgptState) {
    println!("{}", "+-----------------------------+".yellow());
    println!("{}", "TGPT State:".yellow());
    println!("{}", "+-----------------------------+".yellow());
    println!("Initial message:      {:?}", s.inital_message);
    println!("REPL mode:            {:?}", s.repl_mode);
    println!("Piped content:        {:?}", s.piped);
    println!("GPT4 enabled:         {:?}", s.model_4);
    println!("Generate commands:    {:?}", s.gen_cmd);
    println!("Attachment path:      {:?}", s.attach_path);
    println!("Attachment content:   {:?}", s.attach_content);
    println!("Verbose:              {:?}", s.verbose);
    println!("{}", "+-----------------------------+".yellow());
}

pub fn init(conf: config::Config) -> TgptState {
    let args = Args::parse();
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("{}", "No API key set, please set OPENAI_API_KEY".red());
        std::process::exit(-1);
    });

    let client = if args.model_4 {
        ChatGPT::new_with_config(
            api_key.to_string(),
            ModelConfigurationBuilder::default()
                .engine(ChatGPTEngine::Gpt4)
                .build()
                .unwrap(),
        )
        .unwrap()
    } else {
        ChatGPT::new(api_key.to_string()).unwrap()
    };

    let mut state = TgptState {
        inital_message: args.prompt.clone(),
        repl_mode: args.repl_mode || args.prompt.is_none(),
        piped: !atty::is(Stream::Stdin),
        gen_cmd: false,
        model_4: args.model_4,
        verbose: args.verbose,
        attach_path: None,
        attach_content: None,
        conversation: None,
        api_key: None,
        args,
        config: conf,
    };

    let role = role::role_builder(&state);
    print_verbose(
        &format!("{} {}", "built role:".bold(), &role),
        state.verbose,
    );
    let conversation = client.new_conversation_directed(role);
    state.conversation = Some(conversation);

    if state.verbose {
        print_state(&state);
    }

    state
}
