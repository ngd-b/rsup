use local_ip_address::local_ip;
use std::sync::Arc;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    get,
    web::{self},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};

use pkg::Pkg;
use tokio::sync::{mpsc::Receiver, Mutex};
mod socket;
use socket::Ms;
mod api;
/// 获取静态文件路径
pub fn static_file_path() -> String {
    format!("{}/src/static", env!("CARGO_MANIFEST_DIR"))
}

#[get("/")]
async fn index() -> impl Responder {
    let file_path = format!("{}/index.html", static_file_path());

    println!("service statick index html {}", file_path);
    NamedFile::open_async(file_path).await
}

async fn socket_index(
    req: HttpRequest,
    stream: web::Payload,
    ms: web::Data<Arc<Mutex<Ms>>>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    let ms = ms.get_ref().clone();
    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap()
        .to_string();

    actix_web::rt::spawn(async move {
        println!("new connection client's ip : {} ", client_ip);

        Ms::handle_message(ms, session, msg_stream).await;
    });
    Ok(res)
}

pub async fn run(
    data: Arc<Mutex<Pkg>>,
    rx: Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = 8080;

    let socket_ms = Ms {
        data: data.clone(),
        rx,
    };

    let local_ip = local_ip().expect("Could not get local IP address");
    println!("Server running at:");
    println!("  - http://127.0.0.1:{}", port);
    println!("  - http://{}:{}", local_ip, port);

    let ms = web::Data::new(Arc::new(Mutex::new(socket_ms)));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .app_data(web::Data::new(Arc::clone(&data)))
            .service(index)
            .wrap(cors)
            .service(web::scope("/api").configure(api::api_config))
            .service(Files::new("/static", static_file_path()).prefer_utf8(true))
            .app_data(ms.clone())
            .route("/ws", web::get().to(socket_index))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
