use actix_easy_multipart::MultipartFormConfig;
use crate::controller::{img_controller, sys_auth_controller, sys_config_controller, sys_dept_controller, sys_dict_data_controller, sys_dict_type_controller, sys_menu_controller, sys_notice_controller, sys_post_controller, sys_profile_controller, sys_role_controller, sys_user_controller};
use crate::service::CONTEXT;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use actix_web_validator::{Error, JsonConfig};
use actix_web_validator::error::flatten_errors;
use crate::controller::monitor::{server_controller, sys_logininfor_controller, sys_user_online_controller};
use crate::domain::vo::RespVO;

// #[get("/")]
// async fn index() -> impl Responder {
//     NamedFile::open_async("../dist/index.html").await.unwrap()
// }
//
// #[get("/")]
// async fn fav() -> impl Responder {
//     NamedFile::open_async("../dist/f").await.unwrap()
// }

pub fn build_server(base_api: &'static str) -> Server {
    HttpServer::new(|| {
        //定义validate 错误的返回json
        //目前只支持Json，form query path需要再写，目前用不上
        let json_config = JsonConfig::default().error_handler(|err, _req| {
            match err {
                Error::Validate(e) => {
                    let err_str = flatten_errors(&e).iter()
                        .map(|(_, field, err)| { format!("{}", err) })
                        .collect::<Vec<_>>()
                        .join("\n");
                    let resp: RespVO<String> = RespVO {
                        code: 500,
                        msg: Some(err_str),
                        data: None,
                    };
                    actix_web::error::InternalError::from_response(e, resp.resp_json()).into()
                }
                _ => {
                    actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
                }
            }
        });
        let base_api = base_api.to_string();
        App::new()
            //  .wrap(Auth {})
            .app_data(json_config)
            .app_data(web::PayloadConfig::new(50 * 1024 * 1024))
            .app_data(
                MultipartFormConfig::default()
                    .memory_limit(50 * 1024 * 1024)
                    .total_limit(50 * 1024 * 1024),
            )
           // .service(index)
            //.service(fs::Files::new("/assets", "../dist/assets").show_files_listing())
            .service(web::scope(&base_api).service(
                web::scope("/system") //系统应用
                    .configure(sys_menu_controller::init)
                    .configure(sys_profile_controller::init)
                    .configure(sys_user_controller::init)
                    .configure(sys_role_controller::init)
                    .configure(sys_dict_type_controller::init)
                    .configure(sys_dict_data_controller::init)
                    .configure(sys_config_controller::init)
                    .configure(sys_dept_controller::init)
                    .configure(sys_post_controller::init)
                    .configure(sys_notice_controller::init)
            ).service(
                web::scope("/monitor")
                    .configure(sys_logininfor_controller::init)
                    .configure(sys_user_online_controller::init)
                    .configure(server_controller::init)
            ).route("/captchaImage", web::get().to(img_controller::captcha))
                .route(
                    "/login",
                    web::post().to(sys_auth_controller::login),
                )
                .route(
                    "/getInfo",
                    web::get().to(sys_auth_controller::info),
                )
                .route(
                    "/getRouters",
                    web::get().to(sys_menu_controller::routers),
                )
                .route(
                    "/logout",
                    web::post().to(sys_auth_controller::logout),
                )
            )
    })
        .bind(&CONTEXT.config.server_url).unwrap()
        .run()
}
