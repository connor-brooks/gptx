![GPTX Logo](logo.png)

*GPT for the Unix shell*

TGPT (terminal GPT) is a simple CLI program for interacting with OpenAI's GPT APIs. Its goal is to offer a simple extensible component to act as a building block allowing integration of LLMs into existing workflows, scripts and programs. For example, a simple shell script combining TGPT, dmenu and xsel could be used to create a quick dictionary lookup or translation tool.

## Features
* REPL mode
* GPT3/4 model selection
* Processes piped data
* Vi keybindings
* Prompt history
* User defined roles

## Usage
> [!IMPORTANT]
> You must provide your API KEY via the `OPENAI_API_KEY` environment variable, or provide it as `api_key` in `config.toml` You must also define at least a default role in `config.toml`

### Arguments
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

### REPL mode
If TGPT is started with no arguments it will automatically start REPL mode. REPL mode allows you to carry out a conversation with the selected role. Additional REPL mode can be triggered by the `-R` flag, which allows you to provide an initial prompt and still enter REPL mode (e.g, `tgpt -R "what is the capital of France"`)

| Keybinding   | Action           |
| ------------ | ---------------- |
| Return       | Send message     |
| ESC          | Enter vi mode    |
| ALT + Return | Insert newline   |
| UP           | Previous history |
| Down         | Next history     |

### Piped mode
> [!NOTE]
> Currently Piped mode does not support REPL mode. 

Piped mode allows data from any other program to be piped into your selected role whilst also providing an initial prompt. For example:
```
$ echo "dog elephant snake bee whale" | tgpt "sort these by weight"
whale, elephant, dog, snake, bee
```
The output of TGPT can of course be piped or redirected just like any other CLI application. For example: `cat main.rs | tgpt -r rs "add comments to this code" > main.commented.rs`, which would created a commented version of `main.rs`.



## Roles
> [!TIP]
> A role's GPT version can be overridden at runtime using the -4 flag to force GPT4.

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
Which can be called with `tgpt -r rs "how to reverse a vector"`. 
```
tgpt -r rs "how to reverse a vector"
let mut vec = vec![1, 2, 3, 4, 5];
vec.reverse();
```
Alternatively an alias can be set with `alias rsgpt="tgpt -r rs"`.

## Scripting
TGPT attempts to provide only enough functionality to act as a building block for scripts, allowing the user to tailor a custom experience best suited to their unique workflows. Below a few examples are provided.
### Examples
#### cgpt
The `cgpt` example illustrates how to safely make a command generating script. Below is an example of it's usage:
```
$ ./cgpt "resize all images in dir to 100px"
You are about to run: mogrify -resize 100x100 *.jpg
Are you sure? [y/N]: 
```

## Contribute
Any scripts that integrate are welcome, just add your custom scripts to the `contrib` folder and make a pull request. Please ensure your script follows the template!

## Notes
* The project was heavily inspired by TheR1D's [shell_gpt](https://github.com/TheR1D/shell_gpt).
