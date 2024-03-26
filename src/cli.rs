use clap::Parser;

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
