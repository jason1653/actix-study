use actix_web::{get, post, web, HttpResponse, Responder};
use crate::common::api_response::ApiResponse;

#[get("/hello")]
async fn hello() -> impl Responder {
    let response = ApiResponse::success("Hello, world!".to_string());
    HttpResponse::Ok().json(response)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}
