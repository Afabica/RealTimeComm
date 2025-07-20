use actix_web::{web, HttpResponse, Responder, get, Error as ActixError};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection,
};
use crate::components::models::model_mongo::{AppState, Message};
use crate::components::models::model_postgres::{RegisteredUser, RegisterRequest, LoginRequest, Token};
//use crate::components::services::encryption::generate_jwt;
use crate::components::models::chat_group::ChatGroup;
use tokio::sync::Mutex;
use std::sync::Arc;
use sqlx::{PgPool, postgres::PgPoolOptions, Row};
use sqlx::types::Uuid;
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use futures::StreamExt;
use std::env;
use chrono::Utc;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json("Server is running")
}

pub async fn connect_to_postgres() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn connect_to_mongodb() -> Result<Arc<Mutex<Client>>, mongodb::error::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL1").expect("DATABASE_URL1 must be set");
    let client_options = ClientOptions::parse(&url).await?;
    let client = Client::with_options(client_options)?;
    Ok(Arc::new(Mutex::new(client)))
}

pub async fn create_new_collection(mongo_client: Arc<Mutex<Client>>, collection_name: &str) -> mongodb::error::Result<()> {
    let client = mongo_client.lock().await;
    let db = client.database("chatapp");
    db.create_collection(collection_name, None).await?;
    println!("Collection '{}' created!", collection_name);
    Ok(())
}

pub async fn check_mongo_connection(client: Arc<Mutex<Client>>) -> Result<HttpResponse, ActixError> {
    let client = client.lock().await;
    match client.database("admin").run_command(doc! {"ping": 1}, None).await {
        Ok(_) => Ok(HttpResponse::Ok().json("MongoDB is reachable")),
        Err(e) => {
            eprintln!("MongoDB connection failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("MongoDB connection failed"))
        }
    }
}

pub async fn iterate_mongodb_collection(mongo_client: Arc<Mutex<Client>>, collection_name: &str) -> mongodb::error::Result<()> {
    let client = mongo_client.lock().await;
    let db = client.database("chatapp");
    let collection: Collection<Message> = db.collection(collection_name);
    let mut cursor = collection.find(None, None).await?;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => println!("Fetched: {:?}", doc),
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }
    Ok(())
}

pub async fn iterate_postgres_collection(pg_pool: &PgPool, table_name: &str) -> Result<(), sqlx::Error> {
    let query = format!("SELECT * FROM {}", table_name);
    let mut rows = sqlx::query(&query).fetch(pg_pool);

    while let Some(row) = rows.next().await {
        match row {
            Ok(row) => {
                let id: Uuid = row.try_get("id")?;
                let username: String = row.try_get("username")?;
                let password: String = row.try_get("password")?;
                println!("ID: {}, Username: {}, Password: {}", id, username, password);
            }
            Err(e) => eprintln!("Error fetching row: {:?}", e),
        }
    }
    Ok(())
}

pub async fn get_specific_user_information(
    pg_pool: &PgPool,
    username: &str,
) -> Result<RegisterRequest, actix_web::Error> {
    let result = sqlx::query_as::<_, RegisterRequest>(
        "SELECT username, email, password, roles FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pg_pool)
    .await;

    match result {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(actix_web::error::ErrorNotFound("User not found")),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Database error"))
        }
    }
}


pub async fn get_specific_user(state: web::Data<AppState>, account: &str) -> i32 {
    let result = sqlx::query("SELECT account FROM users WHERE account = $1")
        .bind(account)
        .fetch_optional(state.pg_pool.get_ref())
        .await;

    if let Ok(Some(record)) = result {
        let stored_account: &str = record.try_get("account").unwrap_or("");
        if stored_account == account {
            return 1;
        }
    }
    0
}

//
pub async fn simple_authentication(
    state: web::Data<AppState>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse, ActixError> {
    let login = credentials.into_inner();

    // Step 1: Fetch the user by username
    let user_data = match get_specific_user_information(state.pg_pool.get_ref(), &login.username).await {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    let claims = Token {
        sub: login.username.clone(),
        exp: (Utc::now().timestamp() + 3600) as usize,
        roles: user_data.roles.clone(),

    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("Leopard1000".as_bytes()),
    ).map_err(|e| {
        eprintln!("Failed to extract password: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to extract data from collection")
    })?;
//    // Step 2: Compare plain passwords (or use bcrypt if hashed)
    if login.password == user_data.password {
//        Ok(HttpResponse::Ok(token).body("Authentication successful"))
        Ok(HttpResponse::Ok().body("Authentication successful"))
    } else {
        Ok(HttpResponse::Unauthorized().body("Invalid password"))
    }
}

pub async fn simple_registration(
    state: web::Data<AppState>,
    new_cred: web::Json<RegisterRequest>
) -> Result<HttpResponse, ActixError> {
    let form = new_cred.into_inner();

    let query = r#"
        INSERT INTO users(username, password, email, role)
        VALUES($1, $2, $3, $4)
        RETURNING id, username, email, role
    "#;

    let result = sqlx::query(query)
        .bind(&form.username)
        .bind(&form.password)
        .bind(&form.email)
        .bind(&form.roles)
        .fetch_one(state.pg_pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("User registered")),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Registration failed"))
        }
    }
}
