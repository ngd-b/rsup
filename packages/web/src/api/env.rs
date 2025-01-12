use crate::api::ResParams;
use crate::socket::Ms;
use actix_web::{web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};
use utils;

#[derive(Deserialize, Serialize)]
pub struct Env {
    pub name: Option<String>,
    pub version: Option<String>,
    pub path: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct EnvVec {
    pub node: Option<Env>,
    pub npm: Option<Env>,
    pub pnpm: Option<Env>,
    pub yarn: Option<Env>,
}

impl Env {
    pub fn new(name: &str) -> Self {
        let mut env = Env {
            name: None,
            version: None,
            path: None,
        };

        match utils::get_command_info(name) {
            Some((path, version)) => {
                env.name = Some(name.to_string());
                env.version = version;
                env.path = path;
            }
            None => {}
        };

        env
    }
}

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.route("/get", web::get().to(get_env_data));
}

/**
 * 获取环境变量
 * 包括node、npm、pnpm、yarn
 *
 */
async fn get_env_data(_data: web::Data<Ms>) -> impl Responder {
    let env = EnvVec {
        node: Some(Env::new("node")),
        npm: Some(Env::new("npm")),
        pnpm: Some(Env::new("pnpm")),
        yarn: Some(Env::new("yarn")),
    };

    let res = ResParams::ok(env);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
}
