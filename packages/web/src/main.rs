use pkg;
use std::sync::Arc;
use std::thread;

use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use web::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = Arc::new(Mutex::new(pkg::Pkg::new()));

    let (tx, rx) = channel(100);

    let (data_clone, tx_clone) = (data.clone(), tx.clone());

    tokio::spawn(async move {
        thread::sleep(std::time::Duration::from_millis(10 * 1000));

        let mut data = data_clone.lock().await;
        data.name = Some("hboot".to_string());

        println!("send data and update date");
        tx_clone.send(()).await.unwrap();
    });
    run(data, rx).await
}
