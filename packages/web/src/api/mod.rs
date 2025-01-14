use actix_web::web;

//
use dependence::api as api_pgk;
use env::api as api_env;
use serde_derive::{Deserialize, Serialize};
mod dependence;
mod env;

/// 定义接口返回参数结构体
#[derive(Deserialize, Serialize)]
pub struct ResParams<T> {
    success: bool,
    msg: String,
    data: Option<T>,
}
impl<T> ResParams<T> {
    pub fn ok(data: T) -> ResParams<T> {
        ResParams {
            success: true,
            data: Some(data),
            msg: String::from("ok"),
        }
    }
    pub fn err(msg: String) -> ResParams<T> {
        ResParams {
            success: false,
            data: None,
            msg,
        }
    }
}

/// 定义数据接口
pub fn api_config(cfg: &mut web::ServiceConfig) {
    // 环境变量
    cfg.service(web::scope("/env").configure(api_env));
    // 依赖
    cfg.service(web::scope("/pkg").configure(api_pgk));
}
