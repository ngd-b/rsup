use std::sync::{mpsc::Receiver, Arc};

use actix_web::{
    get,
    web::{self},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};

use pkg::Pkg;
use tokio::{sync::Mutex, task::spawn_local};
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
    rx: web::Data<Receiver<()>>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    spawn_local(Ms::handle_message(session, msg_stream, rx));

    Ok(res)
}

pub async fn run(data: Arc<Mutex<Pkg>>, rx: Receiver<()>) {
    let address = "127.0.0.1:8080";

    //    let socket_ms = Ms {};

    println!("Server started at http://{}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&data)))
            .service(index)
            .app_data(web::Data::new(rx))
            .route("/ws", web::get().to(socket_index))
    })
    .bind(address)
    .unwrap()
    .run()
    .await
    .unwrap();
}
