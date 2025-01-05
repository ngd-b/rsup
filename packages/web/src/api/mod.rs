use actix_web::http::Error;
use actix_web::{web, HttpResponse, Responder};
use pkg::package::package_json::{update_dependencies, UpdateParams};
use pkg::package::package_lock;

use serde_derive::{Deserialize, Serialize};

use crate::socket::Ms;

#[derive(Deserialize, Serialize)]
pub struct RelationGraphReq {
    pub name: String,
}
/// 定义接口参数结构体
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReqParams {
    UpdatePkg(UpdateParams),
    // 批量数组更新
    UpdateAllPkg(Vec<UpdateParams>),
    //
    RelationGraph(RelationGraphReq),
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResType {
    Relation(package_lock::PkgInfo),
}
/// 定义接口返回参数结构体
#[derive(Deserialize, Serialize)]
pub struct ResParams {
    success: bool,
    msg: String,
    data: Option<ResType>,
}
impl ResParams {
    fn ok() -> ResParams {
        ResParams {
            success: true,
            data: None,
            msg: String::from("ok"),
        }
    }
    fn err(msg: String) -> ResParams {
        ResParams {
            success: false,
            data: None,
            msg,
        }
    }
}

/// 定义数据接口
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/getData", web::get().to(get_data))
        .route("/updatePkg", web::post().to(update_pkg))
        .route("/graph", web::get().to(relation_graph));
}

/// 获取数据接口
async fn get_data(data: web::Data<Ms>) -> impl Responder {
    let data_clone = data.package.get_pkg().await;

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
    req: web::Json<ReqParams>,
    data: web::Data<Ms>,
) -> Result<impl Responder, Error> {
    // 调用pkg更新依赖
    match &*req {
        ReqParams::UpdatePkg(params) => {
            let data_clone = data.package.get_pkg().await;

            match update_dependencies(data_clone.path.clone(), params.clone()).await {
                Ok(_) => {
                    let res = ResParams::ok();

                    // 更新依赖包
                    let data_clone = data.get_ref().clone();
                    data_clone.package.update_pkg(params.clone()).await.unwrap();
                    // 发送消息更新
                    data.package.sender.lock().await.send(()).await.unwrap();
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
                Err(e) => {
                    eprintln!("update dep err:{:#?}", e.to_string());

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

/// 获取某个依赖包的依赖关系图
///
/// 前端项目安装指定的版本依赖
async fn relation_graph(
    req: web::Query<ReqParams>,
    data: web::Data<Ms>,
) -> Result<impl Responder, Error> {
    match &*req {
        ReqParams::RelationGraph(params) => {
            let data_clone = data.package.get_pkg().await;

            // 调用pkg读取依赖关系图
            let mut pkg = package_lock::Pkg::new(params.name.clone(), data_clone.path.clone());
            match pkg.read_pkg_graph() {
                Ok(_) => {
                    let mut res = ResParams::ok();

                    res.data = Some(ResType::Relation(pkg.pkg_info));
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
                Err(e) => {
                    eprintln!("update dep err:{:#?}", e.to_string());

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
