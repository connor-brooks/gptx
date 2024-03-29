# TGPT
*A terminal GPT program*

TGPT (terminal GPT) is a simple CLI program for interacting with OpenAI's GPT APIs. It's goal is to extensible and easy to integrate into existing workflows. The project was heavily inspired by TheR1D's [shell_gpt](https://github.com/TheR1D/shell_gpt).

## Features
* REPL mode
* GPT3/4 model selection
* Processes piped data
* Vi keybindings
* Prompt history

## Usage
* You must provide your API KEY via the `OPENAI_API_KEY` environment variable, or provide it as `api_key` in `config.toml`
* You must define at least a default role in `config.toml`

```
Usage: tgpt [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  Prompt

Options:
  -r, --role <ROLE>  Role to use [default: default]
  -4, --model4       Force ChatGPT 4 model
  -R, --repl         Activate REPL mode
  -v, --verbose      Verbose output
  -h, --help         Print help
  -V, --version      Print version
```

## Roles
Roles can be defined in the main config file `$HOME/.config/tgpt/config.toml`. They should all follow this format:

```
[role.default]
    version = 4
    prompt = "attempt to assist the user. keep the messages brief when possible"
```

For example, to create a role that only replies in rust code:

```
[role.rs]
    version = 4
    prompt = "only reply in rust code. no english or explanations just output the code"
```
Which can be called with `tgpt -r rs "how to reverse a vector"`. Alternatively an alias can be set with alias `rsgpt="tgpt -r rs"`.
