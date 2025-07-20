use actix_web::{web, HttpResponse, Responder, Error as ActixError};
use axum::routing::get;
use crate::components::models::model_postgres::{LoginRequest, RegisteredUser};
use crate::components::models::model_mongo::AppState;
use crate::components::models::user_profile::UserProfile;


use bcrypt::verify;

//pub async fn fetch_user_profile(
//    state: web::Data<AppState>,
//    credentials: web::Json<LoginRequest>,
//) -> Result<HttpResponse, ActixError> {
//    let login = credentials.into_inner();
//
//    let user_data = match get_specific_user_information(state.pg_pool.get_ref(), &login.username).await {
//        Ok(user) => user,
//        Err(e) => return Err(e),
//    };
//
//    let is_valid = verify(&login.password, &user_data.password)
//        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification failed"))?;
//
//    if !is_valid {
//        return Ok(HttpResponse::Unauthorized().body("Invalid credentials"));
//    }
//
//    let profile = UserProfile {
//        id: None,
//        username: user_data.username,
//        email: user_data.email,
//        roles: user_data.roles,
//    };
//
//    Ok(HttpResponse::Ok().json(profile))
//}

//pub async fn edit_user_profile(
//    state: web::Data<AppState>,
//    credentials: web::Json<LoginRequest>,
//) -> Result<HttpResponse, ActixError> {
//    let login = credentials.into_inner();
//
//    // Fetch user data
//    let user_data = get_specific_user_information(state.clone(), &login.username).await
//        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
//
//    // Verify password
//    let is_valid = verify(&login.password, &user_data.password)
//        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification failed"))?;
//
//    if !is_valid {
//        return Ok(HttpResponse::Unauthorized().body("Invalid credentials"));
//    }
//
//    // Optional: this is your updated profile before deleting the old one
//    let profile = UserProfile {
//        id: Some(user_data.id),
//        username: user_data.username,
//        email: user_data.email,
//        roles: user_data.roles,
//    };
//
//    let query = format!("DELETE FROM profile WHERE id = {:?}", &profile.id);
//
//    // Delete the profile from DB
//    let result = sqlx::query(&query).fetch(state.pg_pool)
//        .execute(&**state.pg_pool)
//        .await
//        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
//
//    // Success response (consider returning updated profile or status)
//    Ok(HttpResponse::Ok().json(profile))
//}
