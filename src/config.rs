use clap::ArgAction;
use clap::Parser;
use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Prompt {
    pub role: String,
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
}

fn parse_system_prompt(arg: &str) -> Result<Vec<Prompt>> {
    let prompt: Vec<Prompt> = serde_json::from_str(arg).unwrap();
    Ok(prompt)
}

impl Default for Config {
    fn default() -> Self {
        // Parse cli args
        let mut config = Config::parse();
        // Check if we even need to read the config file
        if config.api_key.is_some() && config.model.is_some() && config.prompt.is_some() {
            return config;
        }

        // Check the file path
        if config.config_path.is_none() {
            config.config_path = Some(
                dirs::config_dir()
                    .unwrap()
                    .join("gptrs")
                    .join("config.json"),
            );
            if !config.config_path.clone().unwrap().exists() {
                panic!(
                    "Could not find config file {}",
                    config.config_path.unwrap().to_str().unwrap()
                )
            }
        }

        // Read config file
        let config_text =
            read_to_string(config.config_path.clone().unwrap()).unwrap_or_else(|_| {
                panic!(
                    "Could not read config file {}",
                    config.config_path.clone().unwrap().to_str().unwrap()
                )
            });

        let config_file: Config = serde_json::from_str(&config_text).unwrap();
        config.api_key = config.api_key.or(config_file.api_key);
        config.model = config.model.or(config_file.model);
        config.prompt = config.prompt.or(config_file.prompt);

        // Check if we're missing info and panic if we are
        if config.api_key.is_none() {
            panic!(
                "Missing an API key. Supply one in the configuration file or with a CLI argument."
            )
        } else if config.model.is_none() {
            panic!(
                "Missing a model to use. Supply one in the configuration file or with a CLI argument."
            )
        } else if config.prompt.is_none() {
            panic!("Missing a prompt. Supply one in the configuration file or with a CLI argument.")
        }

        if config.debug.is_none() {
            config.debug = Some(false);
        }

        // Annoyingly, the async-openai library only reads from this env var and can't be passed
        // programmatically
        env::set_var("OPENAI_API_KEY", config.api_key.clone().unwrap());

        config
    }
}
