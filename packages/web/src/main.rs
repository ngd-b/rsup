use std::sync::{Arc, Mutex};

use pkg;
use tokio::sync::broadcast;
use web::run;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(pkg::Pkg::new()));

    let (tx, _rx) = broadcast::channel(10);

    run(data, tx).await;
}
