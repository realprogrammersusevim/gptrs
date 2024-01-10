use clap::ArgAction;
use clap::Parser;
use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Prompt {
    pub role: Role,
    pub content: String,
}

#[derive(Deserialize, Serialize, Parser, Debug)]
#[clap(
    author,
    version,
    about = "A TUI to chat with LLMs. Values can be set with CLI args or in the config file."
)]
struct Config {
    #[arg(short = 'k', long, help = "OpenAI API key to use.")]
    api_key: Option<String>,
    #[arg(short, long, help = "OpenAI model to use.")]
    model: Option<String>,
    #[arg(short, long, value_parser = parse_system_prompt, help = "The system prompt for the Chat model")]
    prompt: Option<Vec<Prompt>>,
    #[arg(
        short,
        long,
        help = "Path to the custom configuration file you want to use."
    )]
    config_path: Option<PathBuf>,
    #[clap(short, long, action = ArgAction::SetTrue, help = "Run in debug mode for increased logging.")]
    debug: Option<bool>,
    #[clap(short, long, action = ArgAction::SetTrue, help = "Run offline for testing")]
    offline: Option<bool>,
    #[clap(short, long, action = ArgAction::SetTrue, help = "Use Vim keybindings for text input.")]
    vim: Option<bool>,
}

#[allow(clippy::unnecessary_wraps)]
fn parse_system_prompt(arg: &str) -> Result<Vec<Prompt>> {
    let prompt: Vec<Prompt> = serde_json::from_str(arg).unwrap();
    Ok(prompt)
}

impl Default for Config {
    fn default() -> Self {
        // Parse cli args
        let mut config_cli = Self::parse();

        // Check the file path
        if config_cli.config_path.is_none() {
            config_cli.config_path = Some(
                dirs::config_dir()
                    .unwrap()
                    .join("gptrs")
                    .join("config.json"),
            );
            assert!(
                config_cli.config_path.clone().unwrap().exists(),
                "Could not find config file {}",
                config_cli.config_path.unwrap().to_str().unwrap()
            );
        }

        // Read config file
        let config_text =
            read_to_string(config_cli.config_path.clone().unwrap()).unwrap_or_else(|_| {
                panic!(
                    "Could not read config file {}",
                    config_cli.config_path.clone().unwrap().to_str().unwrap()
                )
            });

        let config_file: Self = serde_json::from_str(&config_text).unwrap_or_else(|_| {
            panic!(
                "Could not parse config file {}",
                config_cli.config_path.clone().unwrap().to_str().unwrap()
            )
        });
        config_cli.api_key = config_cli.api_key.or(config_file.api_key);
        config_cli.model = config_cli.model.or(config_file.model);
        config_cli.prompt = config_cli.prompt.or(config_file.prompt);
        // While the above options will either have a value or be None the flags will
        // either be True if set or False if not
        config_cli.debug = if config_cli.debug.unwrap() {
            config_cli.debug
        } else {
            config_file.debug
        };
        config_cli.offline = if config_cli.offline.unwrap() {
            config_cli.offline
        } else {
            config_file.offline
        };
        config_cli.vim = if config_cli.vim.unwrap() {
            config_cli.vim
        } else {
            config_file.vim
        };

        // Check if we're missing info and panic if we are
        if config_cli.api_key.is_none() {
            panic!(
                "Missing an API key. Supply one in the configuration file or with a CLI argument."
            )
        } else if config_cli.model.is_none() {
            panic!(
                "Missing a model to use. Supply one in the configuration file or with a CLI argument."
            )
        } else if config_cli.prompt.is_none() {
            panic!("Missing a prompt. Supply one in the configuration file or with a CLI argument.")
        }

        if config_cli.debug.is_none() {
            config_cli.debug = Some(false);
        }

        if config_cli.offline.is_none() {
            config_cli.offline = Some(false);
        }

        if config_cli.vim.is_none() {
            config_cli.vim = Some(false);
        }

        config_cli
    }
}

#[derive(Debug, Clone)]
pub struct Final {
    pub api_key: String,
    pub model: String,
    pub prompt: Vec<Prompt>,
    pub debug: bool,
    pub offline: bool,
    pub vim: bool,
}

impl Default for Final {
    fn default() -> Self {
        let config = Config::default();

        // Annoyingly, the async-openai library only reads from this env var and can't be passed
        // programmatically
        env::set_var("OPENAI_API_KEY", config.api_key.clone().unwrap());

        Self {
            api_key: config.api_key.unwrap(),
            model: config.model.unwrap(),
            prompt: config.prompt.unwrap(),
            debug: config.debug.unwrap(),
            offline: config.offline.unwrap(),
            vim: config.vim.unwrap(),
        }
    }
}
