use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use pkg::Pkg;

use serde_derive::{Deserialize, Serialize};
use tokio::sync::Mutex;

/// 定义接口参数结构体
#[derive(Deserialize, Serialize)]
pub struct ReqParams {}

/// 定义接口返回参数结构体
#[derive(Deserialize, Serialize)]
pub struct ResParams {
    success: bool,
    msg: String,
}
impl ResParams {
    fn ok() -> ResParams {
        ResParams {
            success: true,
            msg: String::from("ok"),
        }
    }
    fn err(msg: String) -> ResParams {
        ResParams {
            success: false,
            msg,
        }
    }
}

/// 定义数据接口
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/getData", web::get().to(get_data))
        .route("/updatePkgInfo", web::post().to(update_pkg_info));
}

/// 获取数据接口
async fn get_data(data: web::Data<Arc<Mutex<Pkg>>>) -> impl Responder {
    let data_clone = data.lock().await;

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&data_clone.clone()).unwrap())
}

/// 依赖包更新
///
/// 指定某个依赖包进行更新
///
/// 前端项目安装指定的版本依赖
async fn update_pkg_info(
    info: web::Json<ReqParams>,
    data: web::Data<Arc<Mutex<Pkg>>>,
) -> impl Responder {
    // 调用pkg更新依赖

    let data_clone = data.lock().await;

    match pkg::update_dependencies(data_clone.path.clone()).await {
        Ok(()) => {
            let res = ResParams::ok();

            HttpResponse::Ok().json(serde_json::to_string(&res).unwrap())
        }
        Err(e) => {
            eprintln!("update dep err:{}", e);

            let res = ResParams::err(e.to_string());
            HttpResponse::Ok().json(serde_json::to_string(&res).unwrap())
        }
    }

    // HttpResponse::Ok().json(serde_json::to_string(&res).unwrap())
}
