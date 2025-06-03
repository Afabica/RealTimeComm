//use actix_web::{web, HttpResponse, Responder, get, post, Error as ActixError};
//use crate::components::services::database::get_specific_user_information;
//use crate::components::models::model_mongo::{AppState, Message};
//use crate::components::models::model_postgres::LoginRequest;
//use sqlx::types::Uuid;
//use serde::json;
//
//
//async fn message_sender(state: web::Data<AppState>, request: web::Json<Message>) -> Result <HttpResponse, actix_web::Error> {
//    let message = request.into_inner();
//    let result = get_specific_user(message.id);
//
//    let query = format!("INSERT INTO messages(sender, receiver, content) values ($1, $2, $3)");
//
//    let result = sqlx::query(&query)
//        .bind(&message.sender)
//        .bind(&message.receiver)
//        .bind(&message.content)
//        .fetch_one(state.pg_pool.get_ref())
//        .await
//
//    match result {
//        Ok(_) => Ok(HttpResponse::Ok().body("Message saved!")),
//        Err(e) => {
//            eprintln!("Application error: {:?}", e);
//            Ok(HttpResponse::InternalServerError().body("Message not sended"))
//        }
//    }
//
//
//}
//
//
//async fn message_deleting(state: web::Data<AppState>, request: web::Json<Message>) -> impl Responder {
//    
//}
//
