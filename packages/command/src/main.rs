use clap::Parser;
use command::{run, Commands};

#[derive(Parser, Debug)]
#[command(name = "rsup", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
pub async fn main() {
    let args = Cli::parse();

    // 读取配置文件
    // match Config::read_config() {
    //     Ok(_) => {
    //         println!("读取配置文件成功!")
    //     }
    //     Err(e) => {
    //         eprintln!("读取配置文件失败: {}", e);
    //     }
    // };
    match args.command {
        Commands::Update { .. } | Commands::Config { .. } => run().await,
    }
}
