use pkg;
use std::sync::Arc;
use std::{sync::mpsc::channel, thread};
use tokio::sync::Mutex;
use web::run;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(pkg::Pkg::new()));

    let (tx, rx) = channel();

    let (data_clone, tx_clone) = (data.clone(), tx.clone());

    tokio::spawn(async move {
        thread::sleep(std::time::Duration::from_millis(5 * 1000));

        let mut data = data_clone.lock().await;
        (*data).name = "hboot".to_string();

        tx_clone.send(()).unwrap();
    });
    run(data, rx).await;
}
