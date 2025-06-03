use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sqlx::types::Uuid;

#[derive(Debug, Deserialize, Clone)]
struct MessageForm { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Uuid, 
    pub content: String,
    pub author_id: String,
    pub channel_id: String,
    pub timestamp: DateTime<Utc>,
     
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub  attachements: Option<Vec<Attachment>>, 

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

//#[derive(Debug, Queryable, Serialize, Deserialize)]
//pub struct User {
//    pub id: Uuid,
//    pub username: String,
//    pub email: String,
//    pub display_name: Option<String>,
//    pub password_hash: String,
//    pub profile_image_url: Option<String>,
//    pub created_at: NaiveDateTime,
//    pub updated_at: NaiveDateTime,
//}
//
//#[derive(Debug, Insertable, Serialize, Deserialize)]
//#[diesel(table_name = users)]
//pub struct NewUser {
//    pub username: String,
//    pub email: String,
//    pub display_name: Option<String>,
//    pub password_hash: String,
//    pub profile_image_url: Option<String>, 
//}
