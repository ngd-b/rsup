use std::sync::Arc;

use actix_web::{
    get,
    web::{self},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};

use pkg::Pkg;
use tokio::sync::{mpsc::Receiver, Mutex};
mod socket;
use socket::Ms;

async fn get_data(data: web::Data<Arc<Mutex<Pkg>>>) -> Pkg {
    let locked_data = data.lock().await;

    locked_data.clone()
}
#[get("/")]
async fn index(data: web::Data<Arc<Mutex<Pkg>>>) -> impl Responder {
    let pkg = get_data(data).await;

    HttpResponse::Ok().json(pkg)
}

async fn socket_index(
    req: HttpRequest,
    stream: web::Payload,
    ms: web::Data<Arc<Mutex<Ms>>>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    let ms = ms.get_ref().clone();
    actix_web::rt::spawn(async move {
        Ms::handle_message(ms, session, msg_stream).await;
    });
    Ok(res)
}

pub async fn run(
    data: Arc<Mutex<Pkg>>,
    rx: Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let address = "127.0.0.1:8080";

    let socket_ms = Ms {
        data: data.clone(),
        rx,
    };

    println!("Server started at http://{}", address);

    let ms = web::Data::new(Arc::new(Mutex::new(socket_ms)));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&data)))
            .service(index)
            .app_data(ms.clone())
            .route("/ws", web::get().to(socket_index))
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
