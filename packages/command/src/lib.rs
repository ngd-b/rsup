use clap::Parser;

use config::Options as ConfigOptions;
use update::Options as UpdateOptions;

mod config;
mod update;
#[derive(Parser, Debug)]
pub enum Commands {
    Config {
        #[clap(subcommand)]
        config: ConfigOptions,
    },
    Update {
        #[clap(subcommand)]
        update: UpdateOptions,
    },
}

pub fn run() {
    let cli = Commands::parse();

    match cli {
        Commands::Config { config } => match config {
            ConfigOptions::List => ConfigOptions::list_config(),
            ConfigOptions::Set => todo!(),
            ConfigOptions::Get { key } => ConfigOptions::get_config_value(&key),
            ConfigOptions::Delete => todo!(),
        },
        Commands::Update { update } => match update {
            UpdateOptions::Rsup => todo!(),
            UpdateOptions::Web => todo!(),
        },
    }
}
