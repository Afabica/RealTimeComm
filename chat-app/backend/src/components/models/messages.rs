use serde::{Deserialize,Serialize};
use bson::{oid::ObjectId, DateTime};
use actix_web::{web, HttpResponse, Responder};
use crate::components::models::entry_reactions::{Reaction, ArrayOfEmojis, MemberEntry};
use mongodb::{
    options::ClientOptions,
    Client, Collection,
};
use bson::doc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
 #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
 pub id: Option<ObjectId>,
 pub sender: Uuid,
 pub content: String,
 pub timestamp: DateTime,
 pub reactions: Vec<Reaction>,
}

impl Message {
    pub fn new(sender: Uuid,  content: String, reactions: Vec<Reaction>) -> Self {
        Self {
            id: None,
            sender,
            content,
            timestamp: DateTime::now(),
            reactions: reactions,
        }
    }

    pub async fn delete_by_id(collection: &Collection<Message>, id: ObjectId) -> Result<HttpResponse, actix_web::Error> {
        let filter = doc! {"_id": id};

        match collection.delete_one(filter, None).await {
            Ok(delete_results) => {
                if delete_results.deleted_count == 1 {
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
}
