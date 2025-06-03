////use actix_web::{web, App, HttpServer, dev::{ServiceFactory, ServiceRequest, ServiceResponse}, middleware::Logger};
////use actix_web::body::BoxBody;
////use actix_web::Error;
////use sqlx::PgPool;
////use mongodb::Client;
////use std::sync::Arc;
////use tokio::sync::Mutex;
////use crate::components::models::model_mongo::AppState;
////use crate::components::services::database::{simple_authentication, simple_registration};
////
////// Assuming AppConfig is defined in your code
////pub struct AppConfig;
////
////// Update this function to return a ServiceFactory with the correct associated types
////pub fn create_http_server(
////    mongo_client: Arc<Mutex<Client>>,
////    pg_pool: web::Data<PgPool>,
////) -> impl ServiceFactory<
////    ServiceRequest = ServiceRequest,
////    Response = ServiceResponse<BoxBody>,
////    Error = Error,
////    InitError = (),
////    Config = AppConfig,
////> {
////    App::new()
////        .app_data(web::Data::new(AppState {
////            mongo_client,
////            pg_pool,
////        }))
////        .wrap(Logger::default())
////        .route("/login", web::post().to(simple_authentication))
////        .route("/registration", web::post().to(simple_registration))
////}
//
//// http_server.rs
////use actix_web::{
////    web, App,
////    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
////    Error,
////};
////use actix_web::body::BoxBody;
////use std::sync::Arc;
////use tokio::sync::Mutex;
////use sqlx::PgPool;
////use mongodb::Client;
////use crate::components::models::model_mongo::AppState;
////use crate::components::services::database::{simple_authentication, simple_registration};
////use actix_web::middleware::Logger;
////
////pub fn create_http_server(
////    mongo_client: Arc<Mutex<Client>>,
////    pg_pool: web::Data<PgPool>,
////) -> impl ServiceFactory<
////    ServiceRequest,
////    Config = actix_web::AppConfig,
////    Response = ServiceResponse<BoxBody>,
////    Error = Error,
////    InitError = (),
////> {
////    App::new()
////        .app_data(web::Data::new(AppState {
////            mongo_client,
////            pg_pool,
////        }))
////        .wrap(Logger::default())
////        .route("/login", web::post().to(simple_authentication))
////        .route("/registration", web::post().to(simple_registration))
////}
//use actix_web::{web, App, middleware::Logger};
//use sqlx::PgPool;
//use mongodb::Client;
//use std::sync::Arc;
//use tokio::sync::Mutex;
//
//use crate::components::models::model_mongo::AppState;
//use crate::components::services::database::{simple_authentication, simple_registration};
//
//pub fn create_http_server(
//    mongo_client: Arc<Mutex<Client>>,
//    pg_pool: PgPool,
//) -> impl Fn() -> App<()> {
//    let pg_data = web::Data::new(pg_pool);
//    
//    move || {
//        App::new()
//            .app_data(web::Data::new(AppState {
//                mongo_client: Arc::clone(&mongo_client),
//                pg_pool: pg_data.clone(),
//            }))
//        .wrap(Logger::default())
//        .route("/login", web::post().to(simple_authentication))
//        .route("/registration", web::post().to(simple_registration))
//    }
//}
//
