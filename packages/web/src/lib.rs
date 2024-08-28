use std::net::TcpListener;

use local_ip_address::local_ip;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    get,
    web::{self},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};

use pkg::package::Package;
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
    data: web::Data<Package>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap()
        .to_string();

    let data_clone = data.get_ref().clone();
    actix_web::rt::spawn(async move {
        println!("new connection client's ip : {} ", client_ip);

        Ms::handle_message(data_clone, session, msg_stream).await;
    });
    Ok(res)
}

/// 获取可用的端口号
pub fn check_is_busy_port() -> u16 {
    let mut port = 8888;

    for _ in 0..20 {
        if let Ok(_listener) = TcpListener::bind(("0.0.0.0", port)) {
            break;
        }
        port += 1; // 端口被占用，尝试下一个端口
    }

    port
}

pub async fn run(data: Package) {
    // let port = 8088;
    let port = check_is_busy_port();

    let local_ip = local_ip().expect("Could not get local IP address");
    println!("Server running at:");
    println!("  - http://127.0.0.1:{}", port);
    println!("  - http://{}:{}", local_ip, port);

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        // 服务启动地址
        let service_url = format!("{}:{}", local_ip, port);
        App::new()
            .app_data(web::Data::new(service_url))
            .app_data(web::Data::new(data.clone()))
            .service(index)
            .wrap(cors)
            .service(web::scope("/api").configure(api::api_config))
            .service(Files::new("/static", static_file_path()).prefer_utf8(true))
            .route("/ws", web::get().to(socket_index))
    })
    .bind(format!("0.0.0.0:{}", port))
    .unwrap_or_else(|_| panic!("Could not start server on port:{}", port))
    .run()
    .await;
}
