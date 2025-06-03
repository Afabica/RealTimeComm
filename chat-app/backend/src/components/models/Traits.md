# Trais  

## Fetching with Traits works for different database
For fetching data from different entities (structs) within the same database.
With single function that fetch any of them dynamicallt, regardless of theie specific type.
Example: 

<mark>

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use sqlx::{PgPool, FromRow, query_as};
use std::sync::Arc;

/// ✅ Define a Trait for Fetching Any Entity
#[async_trait]
pub trait EntityFinder: Sized + Send + Sync {
    async fn find_by_field<D>(db: &D, field: &str, value: &str) -> Result<Self, String>;
}

/// ✅ Define Different Entities
#[derive(Serialize, Deserialize, Debug, FromRow)]
struct LoginUser {
    id: i32,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
struct RegisterUser {
    id: i32,
    username: String,
    email: String,
    verified: bool,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
struct AdminUser {
    id: i32,
    username: String,
    email: String,
    role: String,
}

/// ✅ Implement EntityFinder for LoginUser
#[async_trait]
impl EntityFinder for LoginUser {
    async fn find_by_field<PgPool>(pool: &PgPool, field: &str, value: &str) -> Result<Self, String> {
        let query = format!("SELECT * FROM login_users WHERE {} = $1", field);
        let result = sqlx::query_as::<_, LoginUser>(&query)
            .bind(value)
            .fetch_one(pool)
            .await;
        result.map_err(|_| "LoginUser not found".to_string())
    }
}

/// ✅ Implement EntityFinder for RegisterUser
#[async_trait]
impl EntityFinder for RegisterUser {
    async fn find_by_field<PgPool>(pool: &PgPool, field: &str, value: &str) -> Result<Self, String> {
        let query = format!("SELECT * FROM registered_users WHERE {} = $1", field);
        let result = sqlx::query_as::<_, RegisterUser>(&query)
            .bind(value)
            .fetch_one(pool)
            .await;
        result.map_err(|_| "RegisterUser not found".to_string())
    }
}

/// ✅ Implement EntityFinder for AdminUser
#[async_trait]
impl EntityFinder for AdminUser {
    async fn find_by_field<PgPool>(pool: &PgPool, field: &str, value: &str) -> Result<Self, String> {
        let query = format!("SELECT * FROM admin_users WHERE {} = $1", field);
        let result = sqlx::query_as::<_, AdminUser>(&query)
            .bind(value)
            .fetch_one(pool)
            .await;
        result.map_err(|_| "AdminUser not found".to_string())
    }
}

/// ✅ Generic Function to Fetch Any Entity Type
async fn find_entity<T: EntityFinder, D>(
    db: web::Data<D>,
    field: web::Path<(String, String)>,
) -> impl Responder {
    let (field_name, field_value) = field.into_inner();
    match T::find_by_field(&db, &field_name, &field_value).await {
        Ok(entity) => HttpResponse::Ok().json(entity),
        Err(msg) => HttpResponse::NotFound().json(msg),
    }
}

/// ✅ Start Actix Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://user:password@localhost/chat")
        .await
        .expect("Failed to connect to PostgreSQL");

    let db_data = web::Data::new(pool);

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            // ✅ One function works for different entities
            .route("/login_user/{field}/{value}", web::get().to(find_entity::<LoginUser, PgPool>))
            .route("/register_user/{field}/{value}", web::get().to(find_entity::<RegisterUser, PgPool>))
            .route("/admin_user/{field}/{value}", web::get().to(find_entity::<AdminUser, PgPool>))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

</mark>


Explanation of give code:
1. EntityFinder Trait 
- Defines find_by_field() for all user-related entities.
- Allows different entities (LoginUser, RegisterUser)
2. Each Entity Implements EntityFinder 
- LoginUser, RegisterUser, and AdminUser each have their own dataabase table and fetching logic.
3. find_entity() Generic Function 
- Takes any entity that implements EntityFinder.
- Works dynamically for different user types without code duplication.
4. API Endpoints (Same Function, different Entity). 

