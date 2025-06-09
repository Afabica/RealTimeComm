use actix_web::{web, HttpResponse, Responder, get, Error as ActixError};
use sqlx::{PgPool, postgres::PgPoolOptions, Row};
use  chrono::{Utc, DateTime}; 
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthenticationResponse {
    id: Uuid,
    username: String,
    email: String,
    password: String,
    roles: Vec<String>,
    created_at: DateTime<Utc>,
}
