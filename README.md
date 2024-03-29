# TGPT
*A terminal GPT program*

TGPT (terminal GPT) is a simple CLI program for interacting with OpenAI's GPT APIs. Its goal is to provide a simple building block to integrate LLM into existing workflows, combining the power of LLMs with the UNIX philosophy. 

## Features
* REPL mode
* GPT3/4 model selection
* Processes piped data
* Vi keybindings
* Prompt history
* User defined roles

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
Roles can be defined in the main config file `$HOME/.config/tgpt/config.toml`. An example config can be found in `example/config.toml`. All roles should follow this format:

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

## Scripting
tgpt attempts to provide only enough functionality to act as a building block for scripts, allowing the user to tailor a custom experience best suited to their unique workflows. 
### Examples
#### cgpt
todo

## Notes
* The project was heavily inspired by TheR1D's [shell_gpt](https://github.com/TheR1D/shell_gpt).
