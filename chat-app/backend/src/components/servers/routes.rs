//use actix_web::{web, HttpResponse, Responder};
//use crate::components::services::database::{simple_authentication, ping};
//
//pub fn configure_routes(cfg: &mut web::ServiceConfig) {
//    cfg.service(
//        web::scope("/auth")
//        .route("/login", web::post().to(simple_authentication))
//        .route("/ping", web::post().to(ping))
//        .route("/status", web::get().to(|| async { HttpResponse::Ok().json("Auth server is running")}))
//    )
//}
