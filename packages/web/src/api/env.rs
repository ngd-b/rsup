use crate::api::ResParams;
use crate::socket::Ms;
use actix_web::{web, HttpResponse, Responder};
use utils;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.route("/get", web::get().to(get_env_data));
}

/**
 * 获取环境变量
 * 包括node、npm、pnpm、yarn
 *
 */
async fn get_env_data(_data: web::Data<Ms>) -> impl Responder {
    let env = utils::env::EnvVec {
        node: utils::env::Env::new("node"),
        npm: utils::env::Env::new("npm"),
        pnpm: utils::env::Env::new("pnpm"),
        yarn: utils::env::Env::new("yarn"),
    };

    let res = ResParams::ok(env);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
}
