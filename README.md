# GPTrs

A small TUI written in Rust for interacting with GPTs in the terminal.

<img width="1552" alt="Screenshot 2024-02-08 at 5 06 31 PM" src="https://github.com/realprogrammersusevim/gptrs/assets/93488695/7507e07c-936e-41c6-a7ac-0fef49402fc2">

## Installation

1. GPTrs uses Cargo so to install you first need the
   [Rust toolchain](https://rustup.rs).

2. Clone this repository with
   `git clone https://github.com/realprogrammersusevim/gptrs`.

3. Go in to the directory `cd gptrs`.

4. Install the binary with `cargo install --locked --path .`.

5. Move the example config file to the correct path for your OS and edit the
   values.

   - Linux: `$XDG_CONFIG_HOME/gptrs/config.json` or
     `$HOME/.config/gptrs/config.json`
   - macOS: `$HOME/Library/Application Support/gptrs/config.json`
   - Windows: `{FOLDERID_RoamingAppData}\gptrs\config.json`

6. You can now use the `gptrs` tool (as long as you set up Cargo correctly and
   it's in your path)

## Configuration

GPTrs can be configured using CLI options or a configuration file. Options
specified in the CLI will take precedence over those in the config file. The
default paths that GPTrs will search for a config are the XDG base directory and
the XDG user directory specifications on Linux, the Known Folder system on
Windows, and the Standard Directories on macOS.

```bash
$ gptrs --help
A TUI to chat with LLMs. Values can be set with CLI args or in the config file.

Usage: gptrs [OPTIONS]

Options:
  -k, --api-key <API_KEY>          OpenAI API key to use.
  -m, --model <MODEL>              OpenAI model to use.
  -p, --prompt <PROMPT>            The system prompt for the Chat model
  -c, --config-path <CONFIG_PATH>  Path to the custom configuration file you want to use.
  -d, --debug                      Run in debug mode for increased logging.
  -o, --offline                    Run offline for testing
  -v, --vim                        Use Vim keybindings for text input.
  -a, --api-base <API_BASE>        The base URL for the OpenAI API.
  -h, --help                       Print help
  -V, --version                    Print version
```

- The API key is required to access the OpenAI API.
- The model option specifies which model you want to use at the API endpoint.
- The prompt will be put at the beginning of the conversation and allows you to
  tune the model to your preferences.
- If you want to use a different configuration file than the default you can
  also specify that path
- Debug mode shows a debugging window on the right side so you can see all the
  logged events and issues in real time
- Offline mode is only for dev testing. It won't generate any actual LLM
  responses.
- Vim uses Vi keybindings (or most of them at least) instead of the default
  Emacs keybindings for text input.
- While GPTrs defaults to the OpenAI API unless otherwise specified, you can set
  the base URL to any OpenAI compatible endpoint. If you want to use local
  models like Mistral or Llama simply set up [Ollama](https://ollama.com/) and
  start the server. Then set the API base to `http://localhost:11434/v1` and the
  model to the name of the local model you want to use.

## Keybindings

| Action            | Keybinding |
| ----------------- | ---------- |
| Quit              | `C-c`      |
| Submit message    | `C-d`      |
| Reset chat        | `C-r`      |
| Retry request     | `C-t`      |
| Copy last message | `C-x`      |

## Troubleshooting

Something went wrong or not working as expected? First read any error messages
that are shown. If that doesn't work try launching GPTrs with the `-d` option on
at the CLI and see what's happening under the hood. If you still don't know
what's wrong or you're not sure how to fix it submit an issue.

## Planned Features

- Configurable keybindings and colors
- Syntax highlighting
- Chat history saves
