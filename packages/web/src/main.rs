use pkg;
use pkg::package::Package;
use std::thread;

use web::run;

#[tokio::main]
async fn main() {
    // let data = Arc::new(Mutex::new(pkg::Pkg::new()));

    // let (tx, rx) = channel(100);

    // let (data_clone, tx_clone) = (data.clone(), tx.clone());

    let package = Package::new();

    let package_clone = package.clone();
    tokio::spawn(async move {
        thread::sleep(std::time::Duration::from_millis(10 * 1000));

        let mut data = package.pkg.lock().await;
        data.name = Some("hboot".to_string());

        println!("send data and update date");
        package.sender.lock().await.send(()).await.unwrap();
    });

    run(package_clone).await;
}
