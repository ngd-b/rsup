use clap::Parser;
use pkg::{run, Args};
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    run(args).await;
}
