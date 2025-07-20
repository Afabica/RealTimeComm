use sqlx::{PgPool, query, FromRow, Row, postgres::PgPoolOptions};

use tokio::sync::Mutex;
use sqlx::types::Uuid;
use sqlx::query_as;

use bcrypt::{hash, DEFAULT_COST, BcryptError};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use chrono::{Utc, DateTime}; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub sub: String, 
    pub exp: usize,
    pub roles: Vec<String>,

}   

impl Token {
    pub fn new(sub: String, exp: usize, roles: Vec<String>) -> Self {
        Self {
            sub: sub.to_string(),
            exp: exp as usize,
            roles: roles.to_vec(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)] 
pub struct JwtAuth {
    pub user_id: Uuid, 

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt: Option<Token>,
}

#[derive(Debug,FromRow, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub roles: Vec<String>,
}

impl RegisterRequest {
    pub fn new(username: &str, password: &str, email: &str, roles: &str) -> Self { 
        Self {
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            roles: vec![roles.to_string()],
        }
    }

    pub fn to_dto(&self) -> RegisteredUser {
        RegisteredUser {
            id: None,
            username: self.username.clone(),
            password: self.password.clone(),
            email: self.email.clone(), 
            roles: self.roles.clone(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSession {
    pub id: i128,
    pub user_id: i128,

    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisteredUser {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl RegisteredUser {
    pub fn new(id: &Uuid, username: &str, password:&str, email: &str, roles: String ) -> Result<Self, BcryptError> {
        let hashed = hash(password, 12)?;
        Ok(Self {
            id: None,
            username: username.to_string(),
            password: bcrypt::hash(password, 12)?,
            email: email.to_string(),
            roles: vec![roles.to_string()],
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String, 
    pub email: String, 
    pub role: String, 
    pub password: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    pub user_id: i32,
    pub rating: i32,
    pub notifications: bool,
    pub account: String, 
}





