![gptx Logo](logo.png)

gptx (GPT for Unix) is a simple Rust program for interacting with OpenAI's GPT APIs. Its goal is to offer a simple yet extensible component to act as a building block in a Unix CLI workflow, allowing easy integration of LLMs with traditional scripts and programs. For example, a simple shell script combining gptx, dmenu and xsel could be used to create a quick dictionary lookup or translation tool. A collection of community scripts making use of gptx can be found at the bottom of this document.

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

### Installation
To install first run `make build`, then `make install`. This will install gptx to `~/.local/bin` and an example config to `~/.config/gptx/`.

### Arguments
```
Usage: gptx [OPTIONS] [PROMPT]

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

### One-shot mode
This mode is best for simple queries. All that is needed is an inital prompt, for example:
```
$ gptx "What year was i686 released?"
The i686 architecture was released in 1995.
```
Roles can be specified with the `-r` flag:
```
$ gptx -r rs "How to uppercase a string"
let s = "hello";
let upper_s = s.to_uppercase();
```

### REPL mode
If gptx is started with no arguments it will automatically start REPL mode. REPL mode allows you to carry out a conversation with the selected role. Additional REPL mode can be triggered by the `-R` flag, which allows you to provide an initial prompt and still enter REPL mode (e.g, `gptx -R "what is the capital of France"`)

| Keybinding   | Action           |
| ------------ | ---------------- |
| Return       | Send message     |
| ESC          | Enter vi mode    |
| ALT + Return | Insert newline   |
| Up           | Previous history |
| Down         | Next history     |

### Piped mode
> [!NOTE]
> Currently Piped mode does not support REPL mode. 

Piped mode allows data from any other program to be piped into your selected role whilst also providing an initial prompt. For example:
```
$ echo "dog elephant snake bee whale" | gptx "sort these by weight"
whale, elephant, dog, snake, bee
```
The output of gptx can of course be piped or redirected just like any other CLI application. For example: `cat main.rs | gptx -r rs "add comments to this code" > main.commented.rs`, which would created a commented version of `main.rs`.

## Roles
> [!TIP]
> A role's GPT version can be overridden at runtime using the -4 flag to force GPT4.

Roles can be defined in the main config file `$HOME/.config/gptx/config.toml`. An example config can be found in `example/config.toml`. All roles should follow this format:

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
Which can be called with `gptx -r rs "how to reverse a vector"`. 
```
gptx -r rs "how to reverse a vector"
let mut vec = vec![1, 2, 3, 4, 5];
vec.reverse();
```
Alternatively an alias can be set with `alias rsgpt="gptx -r rs"`.

## Scripting
gptx attempts to provide only enough functionality to act as a building block for scripts, allowing the user to tailor a custom experience best suited to their unique workflows. Below a few examples are provided.
### Examples
#### cgpt
The `cgpt` example illustrates how to safely make a command generating script. Below is an example of it's usage:
```
$ cgpt "resize all images in dir to 100px"
You are about to run: mogrify -resize 100x100 *.jpg
Are you sure? [y/N]: 
```
#### defgpt
The `defgpt` is an example of how to use gptx in a non-terminal setting. When bound to a key combo (e.g, `<CTRL> + <ALT> + D`) it allows the user to quickly lookup a dictionary definition of the word in their clipboard, which is displayed via libnotify's `notify-send`. It is intended for this script to be used with Wayland, but can easily be adapted to X by substituting `wl-paste` with `xclip` or `xev`.

## Contributed Scripts / Roles
Any scripts that integrate are welcome, just add your custom scripts to the `contrib` folder, add your script to the table below and make a pull request. Please ensure your script follows the template!
| Name                                                                                   | Description                                 | Author                                              |
| -------------------------------------------------------------------------------------- | ------------------------------------------- | ----------------------------------------------------|
| [cgpt](https://raw.githubusercontent.com/connor-brooks/gptx/main/example/scripts/cgpt) | CLI command generator with execution dialog | [Connor Brooks](https://github.com/connor-brooks)    |
| [defgpt](https://raw.githubusercontent.com/connor-brooks/gptx/main/example/scripts/defgpt) | Display dictionary definitions with `notify-send` | [Connor Brooks](https://github.com/connor-brooks)    |

## Notes
* The project was heavily inspired by TheR1D's [shell_gpt](https://github.com/TheR1D/shell_gpt).
