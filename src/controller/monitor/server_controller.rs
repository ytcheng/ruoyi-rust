use actix_web::{ get, Responder, HttpRequest, web};

use crate::domain::vo::RespVO;
use permit_macro::has_permit;
use crate::util::hardware::get_server_info;

#[get("/server")]
#[has_permit("monitor:server:list")]
pub async fn server_info() -> impl Responder {
    RespVO::from_result(&Ok(get_server_info())).resp_json()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(server_info);
}


