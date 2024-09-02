use actix_web::http::Error;
use actix_web::{web, HttpResponse, Responder};
use pkg::package::package_json::{update_dependencies, UpdateParams};
use pkg::package::Package;
use serde_derive::{Deserialize, Serialize};

/// 定义接口参数结构体
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReqParams {
    UpdatePkg(UpdateParams),
    // 批量数组更新
    UpdateAllPkg(Vec<UpdateParams>),
}

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
        .route("/updatePkg", web::post().to(update_pkg));
}

/// 获取数据接口
async fn get_data(data: web::Data<Package>) -> impl Responder {
    let data_clone = data.get_pkg().await;

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&data_clone.clone()).unwrap())
}

/// 依赖包更新
///
/// 指定某个依赖包进行更新
///
/// 前端项目安装指定的版本依赖
async fn update_pkg(
    info: web::Json<ReqParams>,
    data: web::Data<Package>,
) -> Result<impl Responder, Error> {
    // 调用pkg更新依赖
    match &*info {
        ReqParams::UpdatePkg(params) => {
            let data_clone = data.get_pkg().await;

            match update_dependencies(data_clone.path.clone(), params.clone()).await {
                Ok(_) => {
                    let res = ResParams::ok();

                    // 更新依赖包
                    let data_clone = data.get_ref().clone();
                    data_clone.update_pkg(params.clone()).await.unwrap();
                    // 发送消息更新
                    data.sender.lock().await.send(()).await.unwrap();
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
                Err(e) => {
                    eprintln!("update dep err:{}", e);

                    let res = ResParams::err(e.to_string());
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
            }
        }
        _ => {
            let res = ResParams::err("Invalid request parameters".to_string().to_string());
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&res).unwrap()))
        }
    }
}
