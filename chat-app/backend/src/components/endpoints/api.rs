//use axum::{Json, etract::Path}
//use serde::{Serialize, Deserialize};
//
//#[derive(Serialize, Deserialize)]
//pub struct User {
//    pub id: u32,
//    pub username: String,
//    pub password: String,
//}
//
//pub async fn get_user(Path(user_id): Path<u32>) -> Json<User> {
//    Json(User {
//        id: user_id,
//        username: format!("User {}", user_id),
//    })
//}
//
//pub async fn create_user(Json(payload): Json<User>) -> Json<User> {
//    Json(payload)
//}
//
