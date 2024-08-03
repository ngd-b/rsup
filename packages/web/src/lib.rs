use std::sync::{Arc, Mutex};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use pkg::Pkg;
use tokio::sync::broadcast;

fn get_data(data: web::Data<Arc<Mutex<Pkg>>>) -> Pkg {
    let locked_data = data.lock().unwrap();

    locked_data.clone()
}
#[get("/")]
async fn index(data: web::Data<Arc<Mutex<Pkg>>>) -> impl Responder {
    let pkg = get_data(data);

    let json = match serde_json::to_string_pretty(&pkg) {
        Ok(json) => json,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to serialize data: {}", e))
        }
    };
    HttpResponse::Ok().json(json)
}

async fn listen_update_data(tx: broadcast::Sender<Pkg>, mut rx: broadcast::Receiver<Pkg>) {
    loop {
        match rx.recv().await {
            Ok(data) => {
                if let Err(e) = tx.send(data) {
                    println!("Failed to send data: {:?}", e);
                }
            }
            Err(e) => {
                println!("Failed to receive data: {:?}", e);
                break;
            }
        }
    }
}

pub async fn run(data: Arc<Mutex<Pkg>>, tx: broadcast::Sender<Pkg>) {
    let (_tx_clone, rx) = broadcast::channel(10);

    tokio::spawn(listen_update_data(tx, rx));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&data)))
            .service(index)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}
