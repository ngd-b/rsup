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
use rand::{thread_rng, Rng};
use socket::{ConnId, Ms};
mod api;
use config::Config;
use tokio::try_join;
use webbrowser;

#[get("/")]
async fn index() -> impl Responder {
    let config = Config::get_config().await;

    let file_path = format!("{}/index.html", config.web.static_dir);

    println!("the service static file path is : {}", file_path);
    NamedFile::open_async(file_path).await
}

/// websocket 处理函数
async fn socket_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Ms>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap()
        .to_string();

    // 保存链接实例
    let id = thread_rng().gen::<ConnId>();
    {
        let mut data_clone = data.connectors.lock().await;

        data_clone.insert(id, session.clone());
    }

    let data_clone = (**data).clone();

    actix_web::rt::spawn(async move {
        println!(
            "new connection client's ip : {}. the client_id is {} ",
            client_ip, id
        );

        Ms::handle_message(data_clone.package, session, msg_stream).await;
    });
    Ok(res)
}

/// 获取可用的端口号
pub fn check_is_busy_port(port: u16) -> u16 {
    // let mut port = 8888;
    let mut port = port;

    for _ in 0..20 {
        if let Ok(_listener) = TcpListener::bind(("0.0.0.0", port)) {
            break;
        }
        port += 1; // 端口被占用，尝试下一个端口
    }

    port
}

pub async fn run(data: Package) {
    let config = Config::get_config().await;
    // let port = 8088;
    let port = check_is_busy_port(config.web.port);

    let local_ip = local_ip().expect("Could not get local IP address");

    // 启动浏览器
    let service_url = format!("http://{}:{}", local_ip, port);
    if webbrowser::open(&service_url).is_ok() {
        println!("Server running at:");
        println!("  - http://127.0.0.1:{}", port);
        println!("  - http://{}:{}", local_ip, port);
    };

    // 创建socket实例
    let ms = Ms::new(data);
    // 启动channel数据监听服务
    let mut ms_clone = ms.clone();
    tokio::task::spawn(async move {
        ms_clone.handle_receiver_msg().await;
    });

    let static_file_path = config.web.static_dir.clone();
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .app_data(web::Data::new(ms.clone()))
            .service(index)
            .wrap(cors)
            .service(web::scope("/api").configure(api::api_config))
            .service(Files::new("/static", static_file_path.clone()).prefer_utf8(true))
            .route("/ws", web::get().to(socket_index))
    })
    .workers(5)
    .bind(format!("0.0.0.0:{}", port))
    .unwrap_or_else(|_| panic!("Could not start server on port:{}", port))
    .run();

    // try_join!(receiver_server).unwrap();
    try_join!(server).unwrap();
}
