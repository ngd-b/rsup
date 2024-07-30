use clap::{Parser, Subcommand};
use pkg;
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Pkg(pkg::Args),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Pkg(args) => pkg::run(args).await,
    }
}
