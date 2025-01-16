use actix_web::http::Error;
use actix_web::{web, HttpResponse, Responder};
use pkg::manager::pkg_lock;
use pkg::package::package_json::{
    remove_dependencies, update_dependencies, RemoveParams, UpdateParams,
};

use crate::api::ResParams;
use crate::socket::Ms;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RelationGraphReq {
    pub name: String,
}
/// 定义接口参数结构体
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ReqParams {
    UpdatePkg(UpdateParams),
    // 批量数组更新
    UpdateAllPkg(Vec<UpdateParams>),
    // 删除
    // 目前接受一个name ，直接使用RelationGraphReq的结构题
    RemovePkg(RemoveParams),
    //
    RelationGraph(RelationGraphReq),
}

/// 定义数据接口
pub fn api(cfg: &mut web::ServiceConfig) {
    // 依赖
    cfg.route("/get", web::get().to(get_data))
        .route("/update", web::post().to(update_pkg))
        .route("/graph", web::get().to(relation_graph))
        .route("/remove", web::post().to(remove_pkg));
}

/// 获取数据接口
async fn get_data(data: web::Data<Ms>) -> impl Responder {
    let data_clone = data.package.get_pkg().await;

    let res = ResParams::ok(data_clone.clone());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
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
    println!("receive 【update_pkg】 req param {:?}", req);
    // 调用pkg更新依赖
    match &*req {
        ReqParams::UpdatePkg(params) => {
            let data_clone = data.package.get_pkg().await;

            let mut update_params = params.clone();
            // 如果是切换依赖类型，选择先删除、在安装
            if params.is_change.unwrap_or(false) {
                // 切换依赖类型，先删除再安装
                update_params.is_dev = !params.is_dev;
                // 删除依赖
                let file_path = data_clone.path.clone();
                let manager_name = data_clone.manager_name.clone().unwrap();
                // 先删除依赖
                let remove_params = RemoveParams {
                    name: params.name.clone(),
                    is_dev: params.is_dev.clone(),
                };
                if let Err(e) = remove_dependencies(file_path, remove_params, manager_name).await {
                    eprintln!("update dep err:{:#?}", e.to_string());

                    let res = ResParams::<String>::err(e.to_string());
                    return Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()));
                };
            }

            match update_dependencies(
                data_clone.path.clone(),
                update_params,
                data_clone.manager_name.unwrap(),
            )
            .await
            {
                Ok(_) => {
                    let res = ResParams::ok("");

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

                    let res = ResParams::<String>::err(e.to_string());
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
            }
        }
        err => {
            println!("Invalid request parameters:{:#?}", err);
            let res =
                ResParams::<String>::err("Invalid request parameters".to_string().to_string());
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
    println!("receive 【relation_graph】 req param {:?}", req);
    match &*req {
        ReqParams::RelationGraph(params) => {
            let data_clone = data.package.get_pkg().await;

            // 调用pkg读取依赖关系图
            let mut pkg = pkg_lock(
                &data_clone.manager_name.unwrap(),
                params.name.clone(),
                data_clone.path.clone(),
            );
            match pkg.read_pkg_graph() {
                Ok(pkg_info) => {
                    let res = ResParams::ok(pkg_info);

                    // res.data = Some(ResType::Relation(pkg.pkg_info));
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
                Err(e) => {
                    eprintln!("update dependence err:{:#?}", e.to_string());

                    let res = ResParams::<String>::err(e.to_string());
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
            }
        }
        err => {
            println!("Invalid request parameters:{:#?}", err);
            let res =
                ResParams::<String>::err("Invalid request parameters".to_string().to_string());
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&res).unwrap()))
        }
    }
}

/// 删除依赖包
///
/// # Arguments
/// * req 请求参数
/// * data 请求上下文
///
/// # Returns
/// * Result<HttpResponse, Box<dyn Error>>
///
///
async fn remove_pkg(
    req: web::Json<ReqParams>,
    data: web::Data<Ms>,
) -> Result<impl Responder, Error> {
    println!("receive 【remove_pkg】 req param {:?}", req);
    // 调用pkg更新依赖
    match &*req {
        ReqParams::RemovePkg(params) => {
            let data_clone = data.package.get_pkg().await;

            match remove_dependencies(
                data_clone.path.clone(),
                params.clone(),
                data_clone.manager_name.unwrap(),
            )
            .await
            {
                Ok(_) => {
                    let res = ResParams::ok("");

                    // 更新依赖包
                    let data_clone = data.get_ref().clone();
                    data_clone.package.remove_pkg(params.clone()).await.unwrap();
                    // 发送消息更新
                    data.package.sender.lock().await.send(()).await.unwrap();
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
                Err(e) => {
                    eprintln!("remove dependence err:{:#?}", e.to_string());

                    let res = ResParams::<String>::err(e.to_string());
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap()))
                }
            }
        }
        err => {
            println!("Invalid request parameters:{:#?}", err);
            let res =
                ResParams::<String>::err("Invalid request parameters".to_string().to_string());
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&res).unwrap()))
        }
    }
}
