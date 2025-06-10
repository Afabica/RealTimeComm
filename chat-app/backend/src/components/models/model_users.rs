use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use mongodb::{Client, Collection};
use futures::stream::StreamExt;
use std::sync::Arc;
use crate::components::models::chat_group::ChatGroup;
use tokio::sync::Mutex;
use bson::doc;

#[derive(Debug, Deserialize, Clone)]
struct MessageForm { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,  // <-- Changed to Option<Uuid>
    pub content: String,
    pub author_id: String,
    pub channel_id: String,
    pub timestamp: DateTime<Utc>,
     
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,  // <-- fixed typo

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub url: String,
    pub filename: String,
    pub size: u64, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub color: Option<u32>,
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: Uuid, 
    pub email: String, 
    pub display_name: Option<String>,
    pub password_hash: String, 
    pub profile_image_url: Option<String>,
    pub user_groups: Vec<u16>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserData {
    pub fn create_user_data(id: Uuid, email: String, display_name: Option<String>, password_hash: String, profile_image_url: Option<String>) -> Self {
        Self {
            id,  // can omit `id: id`
            email,
            display_name,  // fixed typo here
            password_hash,
            profile_image_url,
            user_groups: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

//    pub async fn fetch_current_group(id: Uuid, mongo_client: Arc<Mutex<Client>>) -> Result<Option<ChatGroup>, mongodb::error::Error> {
//        let client = mongo_client.lock().await;
//        let db = client.database("chatapp");
//        let collection: Collection<ChatGroup> = db.collection("groups"); 
//        
//        let filter = doc! { "_id": id };
//        let chat_group = collection.find_one(filter, None).await?;
//        
//        Ok(chat_group)
//    }
}

