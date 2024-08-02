use clap::Parser;
use pkg::{run, Args};
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match run(args).await {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(e) => {
            eprintln!("Error reading package.json: {}", e)
        }
    };
}
