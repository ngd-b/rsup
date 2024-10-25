use std::error::Error;

use clap::Parser;
use config::Config;
use toml;

#[derive(Parser, Debug)]
pub enum Options {
    List,
    Set {
        key: String,
        value: String,
    },
    #[clap(name = "get", about = "Get config value")]
    Get {
        key: String,
    },
    Delete,
}

impl Options {
    /// List config
    ///
    pub async fn list_config() -> Result<(), Box<dyn Error>> {
        let config = Config::get_config().await;

        match toml::to_string_pretty(&*config) {
            Ok(data) => {
                println!("{}", data);
                Ok(())
            }
            Err(e) => {
                eprintln!("{}", e);
                Err("Error: Unable to convert config to string".into())
            }
        }
    }
    pub async fn get_config_value(key: &str) -> Result<(), Box<dyn Error>> {
        let config = Config::get_config().await;
        match config.get(key) {
            Some(value) => {
                println!("{}", value);
                Ok(())
            }
            None => {
                eprintln!("Key not found");
                Err("Key not found".into())
            }
        }
    }
    pub async fn set_config_value(key: &str, value: String) -> Result<(), Box<dyn Error>> {
        println!("{}: {}", key, value);
        // let config = Config::get_config();
        let mut config = Config::get_mut_config().await;

        match config.set(key, value) {
            Ok(_) => {
                println!(
                    "Set successfully,new value is:{:#?}",
                    config.get(key).unwrap()
                );
                Ok(())
            }
            Err(e) => {
                eprintln!("{}", e);
                Err("Unable to set config".into())
            }
        }
    }
}
