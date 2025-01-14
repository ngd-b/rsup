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
    let env = utils::rs_env::EnvVec {
        node: utils::rs_env::Env::new("node"),
        npm: utils::rs_env::Env::new("npm"),
        pnpm: utils::rs_env::Env::new("pnpm"),
        yarn: utils::rs_env::Env::new("yarn"),
    };

    let res = ResParams::ok(env);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
}
