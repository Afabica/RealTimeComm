use serde::{Deserialize, Serialize};
use rocket::form::Options;
use mongodb::options::ClientOptions;
use std::sync::Arc;
use actix_web::{web, Responder, HttpResponse};
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use mongodb::Collection;
use sqlx::{PgPool, query};
use tokio::sync::Mutex;
use sqlx::types::Uuid;
use actix::prelude::*;

use crate::components::services::clserver_entities::ChatServer;
//use chrono::{Utc, DateTime};


//use jsonwebtoken::{decode, encode, Header, DecodingKey,EncodingKey, Validation};


// Application Settings - Defines active mode (Civil or Military)
#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub mode: String, // "civil" or "military"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String, 
    pub email: String, 
    pub role: String, 
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub mongo_client: Arc<Mutex<mongodb::Client>>,
    pub pg_pool: web::Data<PgPool>,
    pub chat_server: Addr<ChatServer>
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub message: String,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Claims {
    pub sub: String, 
    pub exp: usize,
}


// Registered User Entity
//#[derive(Debug, Serialize, Deserialize)]
//pub struct RegisteredUser {
//    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//    pub id: Option<ObjectId>,
//    pub username: String,
//    pub email: String,
//    pub password: String, // Hashed password
//    pub roles: String,
//   pub created_at: DateTime,
//    pub updated_at: DateTime,
//}
//
//impl RegisteredUser {
//    pub fn new(username: &str, email: &str, password: &str, roles: String) -> Self {
//        let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");
//        Self {
//            id: None,
//            username: username.to_string(),
//            email: email.to_string(),
//            password: hashed_password,
//            roles: roles.to_string(),
//            created_at: DateTime:,
//            updated_at: DateTime::now(),
//        }
//    }
//}


// Role Entity
#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub role: String,
    pub permissions: Vec<String>,
}

impl Role {
    pub fn new(role: &str, permissions: Vec<String>) -> Self {
        Self {
            id: None,
            role: role.to_string(),
            permissions,
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }
}

// Civil Mode Data Entity
//
#[derive(Debug, Serialize, Deserialize)]
pub struct CivilData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub infrastructure_report: String,
    pub safety_guidelines: String,
}

// Military Mode Data Entity
#[derive(Debug, Deserialize)]
pub struct MilitaryData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub classified_report: String,
    pub security_clearance_level: String,
}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct  LoginSession {
//    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//    pub id: Option<ObjectId>,
//    pub user_id: ObjectId,
//    pub token: String, 
//    pub expires_at: DateTime, 
//    pub created_at: DateTime,
//}
//
//impl LoginSession {
//    pub fn new(user_id: ObjectId, token: &str, expires_at: DateTime) -> Self {
//        Self {
//            id: None, 
//            user_id, 
//            token: token.to_string(),
//            expires_at, 
//            created_at: DateTime::now(),
//        }
//    }
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: Option<ObjectId>,
    pub sub: String, 
    pub exp: usize, 
}

impl Token {
    pub fn new(id: Option<ObjectId>, sub: &str, exp: &usize) -> Self {
        Self {
        id, 
        sub: sub.to_string(),
        exp: *exp,
        }
    }     
}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct Message {
//    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//    pub id: Option<ObjectId>,
//    pub sender: String, 
//    pub receiver: String,
//    pub content: String,
//    pub timestamp: DateTime<Utc>,
//}
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, 
    pub sender: Uuid,
    pub receiver: Uuid,
    pub content: String,
    pub timestamp: DateTime,
} 

impl Message {
    pub fn new(sender: Uuid, receiver: Uuid, content: String) -> Self {
        Self {
            id: None,
            sender,
            receiver,
            content,
            timestamp: DateTime::now(),
        }
    }

    pub async fn delete_by_id(collection: &Collection<Message>, id: ObjectId,) -> Result<HttpResponse, actix_web::Error> {
        let filter = doc! {"_id": id};
         
        match collection.delete_one(filter, None).await {
            Ok(delete_result) => {
                if delete_result.deleted_count == 1 {
                 Ok(HttpResponse::Ok().body("Message deleted"))
                } else {
                 Ok(HttpResponse::NotFound().body("Message not found")) 
                }
            }
            Err(err) => {
                eprintln!("MongoDB deletion error: {:?}", err);
                Ok(HttpResponse::InternalServerError().body("Failed to delete message"))
            }
        }
    }       
    
//    pub fn validate_jwt(&self) -> bool {
//        match decode::<Token> (
//            &self.jwt,
//            &DecodingKey::from_secret("secret".as_ref()),
//            &Validation::default() 
//        ) {
//            Ok(_) => true, 
//            Err(_) => false,
//        }
//    }
}

