use actix_web::http::Error;
use actix_web::{web, HttpResponse, Responder};
use pkg::manager::{pkg_lock, PkgInfo};
use pkg::package::package_info::PkgInfo as DepPkgInfo;
use pkg::package::package_json::{
    batch_update_dependencies, quick_install_dependencies, remove_dependencies,
    update_dependencies, QuickInstallParams, RemoveParams, UpdateParams,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::{self};

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
    // 删除
    // 目前接受一个name ，直接使用RelationGraphReq的结构题
    RemovePkg(RemoveParams),
    //
    RelationGraph(RelationGraphReq),
    // 一键安装所有依赖
    QuickInstall(QuickInstallParams),
    // 批量更新
    BatchUpdatePkg(Vec<UpdateParams>),
}

/// 定义数据接口
pub fn api(cfg: &mut web::ServiceConfig) {
    // 依赖
    cfg.route("/get", web::get().to(get_data))
        .route("/update", web::post().to(update_pkg))
        .route("/graph", web::get().to(relation_graph))
        .route("/realtion", web::get().to(relation_data))
        .route("/remove", web::post().to(remove_pkg))
        .route("/quickInstall", web::post().to(quick_install))
        .route("/batchUpdate", web::post().to(batch_update_pkg))
        .route("/reload", web::post().to(relod_pkg));
}

async fn relod_pkg(data: web::Data<Ms>) -> impl Responder {
    let data_clone = data.package.get_pkg().await;

    //  异步更新依赖包信息
    task::spawn(async move {
        pkg::read_latest_pkgs(data_clone.path, data.package.clone()).await;
    });

    let res = ResParams::ok("");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
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
            let pkg = pkg_lock(data_clone.manager_name.unwrap(), data_clone.path.clone());
            match pkg.read_pkg_graph(params.name.clone()) {
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

/// 获取某个依赖包的依赖关系图
///
/// 前端项目安装指定的版本依赖
async fn relation_data(data: web::Data<Ms>) -> Result<impl Responder, Error> {
    println!("receive 【relation_graph】 req param");

    let data_clone = data.package.get_pkg().await;

    // 读取lock文件
    let (manager_name, pkg_path) = (data_clone.manager_name.unwrap(), data_clone.path);
    let pkg = Arc::new(pkg_lock(manager_name.clone(), pkg_path.clone()));

    let vec_pkg = Arc::new(Mutex::new(HashMap::<String, PkgInfo>::new()));

    // 提取一个公共方法
    let read_pkg_task = |data: HashMap<String, DepPkgInfo>| {
        data.iter()
            .map(|(name, _)| {
                let pkg_clone = pkg.clone();
                let vec_pkg_clone = vec_pkg.clone();
                let name_clone = name.clone();
                task::spawn(async move {
                    let info = pkg_clone.read_pkg_graph(name_clone.clone()).unwrap();

                    let mut vec_pkg = vec_pkg_clone.lock().await;
                    vec_pkg.insert(name_clone, info);
                })
            })
            .collect::<Vec<_>>()
    };

    // 并行
    let dep_task = read_pkg_task(data_clone.dependencies);

    let dev_dep_task = read_pkg_task(data_clone.dev_dependencies);

    let _dep = futures_util::future::join_all(dep_task).await;
    let _dev_dep = futures_util::future::join_all(dev_dep_task).await;

    let res_data = vec_pkg.lock().await;
    let res = ResParams::ok(res_data.clone());

    // res.data = Some(ResType::Relation(pkg.pkg_info));
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap()))
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

/// 快速一键安装项目依赖
///
/// # Arguments
/// * req 请求参数
/// * data 请求上下文
///
/// # Returns
/// * Result<HttpResponse, Box<dyn Error>>
///
///
async fn quick_install(
    req: web::Json<ReqParams>,
    data: web::Data<Ms>,
) -> Result<impl Responder, Error> {
    println!("receive 【quick_install】 req param {:?}", req);
    // 调用pkg更新依赖
    match &*req {
        ReqParams::QuickInstall(params) => {
            let data_clone = data.package.get_pkg().await;

            match quick_install_dependencies(data_clone.path.clone(), params.clone()).await {
                Ok(_) => {
                    let res = ResParams::ok("");
                    // 更新当前确定使用的包管理工具
                    let mut data_clone = data.package.pkg.lock().await;
                    data_clone.manager_name = Some(params.manager_name.clone());

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

/// 批量更新依赖包
///
/// # Arguments
/// * req 请求参数
/// * data 请求上下文
///
/// # Returns
/// * Result<HttpResponse, Box<dyn Error>>
///
async fn batch_update_pkg(
    req: web::Json<ReqParams>,
    data: web::Data<Ms>,
) -> Result<impl Responder, Error> {
    println!("receive 【batch_update_pkg】 req param {:?}", req);
    // 调用pkg更新依赖
    match &*req {
        ReqParams::BatchUpdatePkg(params) => {
            let data_clone = data.package.get_pkg().await;

            match batch_update_dependencies(
                data_clone.path.clone(),
                params.clone(),
                data_clone.manager_name.unwrap(),
            )
            .await
            {
                Ok(names) => {
                    let res = ResParams::ok(names.clone());

                    // 更新依赖包
                    for param in params {
                        let data_clone = data.get_ref().clone();
                        if names.contains(&param.name) {
                            data_clone.package.update_pkg(param.clone()).await.unwrap();
                        }
                    }

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
