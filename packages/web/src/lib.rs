use std::sync::{mpsc::Receiver, Arc};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use pkg::Pkg;
use tokio::sync::Mutex;

async fn get_data(data: web::Data<Arc<Mutex<Pkg>>>) -> Pkg {
    let locked_data = data.lock().await;

    locked_data.clone()
}
#[get("/")]
async fn index(data: web::Data<Arc<Mutex<Pkg>>>) -> impl Responder {
    let pkg = get_data(data).await;

    HttpResponse::Ok().json(pkg)
}

pub async fn run(data: Arc<Mutex<Pkg>>, _rx: Receiver<()>) {
    let address = "127.0.0.1:8080";

    println!("Server started at http://{}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&data)))
            .service(index)
    })
    .bind(address)
    .unwrap()
    .run()
    .await
    .unwrap();
}
