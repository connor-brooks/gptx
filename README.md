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
```
Usage: tgpt [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  Prompt

Options:
  -r, --repl                 Activate REPL mode
  -e, --echo                 Activate echo
  -c, --command              Generate shell commands
  -4, --model4               Enable ChatGPT 4 model
  -a, --attach <ATTACHMENT>  Attach a file
  -v, --verbose              Verbose output
  -h, --help                 Print help
  -V, --version              Print version
```
