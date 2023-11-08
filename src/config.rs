use clap::Parser;
use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Prompt {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Parser, Debug)]
pub struct Config {
    #[arg(short = 'k', long)]
    pub api_key: Option<String>,
    #[arg(short = 'b', long)]
    pub api_base: Option<String>,
    #[arg(short, long)]
    pub model: Option<String>,
    #[arg(value_parser = parse_system_prompt)]
    pub prompt: Option<Vec<Prompt>>,
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
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
        if config.api_key.is_some()
            && config.api_base.is_some()
            && config.model.is_some()
            && config.prompt.is_some()
        {
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
        config.api_base = config.api_base.or(config_file.api_base);
        config.model = config.model.or(config_file.model);
        config.prompt = config.prompt.or(config_file.prompt);

        // Check if we're missing info and panic if we are
        if config.api_key.is_none() {
            panic!(
                "Missing an API key. Supply one in the configuration file or with a CLI argument."
            )
        } else if config.api_base.is_none() {
            panic!(
                "Missing an API base URL. Supply one in the configuration file or with a CLI argument."
            )
        } else if config.model.is_none() {
            panic!(
                "Missing a model to use. Supply one in the configuration file or with a CLI argument."
            )
        } else if config.prompt.is_none() {
            panic!("Missing a prompt. Supply one in the configuration file or with a CLI argument.")
        }

        config
    }
}
