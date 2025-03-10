use rsup_utils::get_version;

#[tokio::main]
async fn main() {
    let v = get_version("rsup".to_string(), Option::None).await;

    match v {
        Ok(name) => {
            println!("version: {}", name)
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
