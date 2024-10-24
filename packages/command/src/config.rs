use clap::Parser;
use config::Config;
use toml;

#[derive(Parser, Debug)]
pub enum Options {
    List,
    Set,
    Get { key: String },
    Delete,
}

impl Options {
    /// List config
    ///
    pub fn list_config() {
        let config = Config::get_config();

        match toml::to_string_pretty(config) {
            Ok(data) => {
                println!("{}", data);
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
    pub fn get_config_value(key: &str) {
        let config = Config::get_config();
        match config.get(key) {
            Some(value) => println!("{}", value),
            None => eprintln!("Key not found"),
        }
    }
}
