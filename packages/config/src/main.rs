use rsup_config::Config;

#[tokio::main]
async fn main() {
    let config = Config::write_config().await.unwrap();

    println!("{:?}", config)
}
