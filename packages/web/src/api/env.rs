use crate::api::ResParams;
use crate::socket::Ms;
use actix_web::{web, HttpResponse, Responder};
use rs_utils;
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.route("/get", web::get().to(get_env_data));
}

/**
 * 获取环境变量
 * 包括node、npm、pnpm、yarn
 *
 */
async fn get_env_data(_data: web::Data<Ms>) -> impl Responder {
    let env = rs_utils::env::EnvVec {
        node: rs_utils::env::Env::new("node"),
        npm: rs_utils::env::Env::new("npm"),
        pnpm: rs_utils::env::Env::new("pnpm"),
        yarn: rs_utils::env::Env::new("yarn"),
    };

    let res = ResParams::ok(env);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
}
