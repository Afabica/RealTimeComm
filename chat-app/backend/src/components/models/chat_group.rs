use serde::{Deserialize, Serialize};
use bson::{oid::ObjectId, DateTime};
use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    options::ClientOptions,
    Client, Collection,
};
//use chrono::{Utc, DateTime};
use crate::components::models::messages::Message;
use crate::components::models::entry_reactions::MemberEntry;
use uuid::Uuid;

#[derive(Debug,Serialize, Deserialize)]
pub struct ChatGroup {
    #[serde(rename = "_id")]
    pub id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub members: Vec<MemberEntry>,
    pub messages: Vec<Message>,
    pub created_at: DateTime,
}

impl ChatGroup {
    pub fn newChatGroup(id: ObjectId, name: String, description: Option<String>, created_by: Uuid, members: Vec<MemberEntry>) -> Self {
        Self {
            id: None, 
            name: name.to_string(),
            description: description,
            created_by: created_by,
            members: members,
            messages: Vec::new(),
            created_at: DateTime::now()
        }
    }

    pub fn getMemberOfGroup(members: Vec<MemberEntry>) -> Self {

    }
}


//#[derive(Message)]
//#[rtype(result = "()")]
//struct ChatMessage(pub String);
//
//#[derive(Message)]
//#[rtype(usize)]
//struct Connect {
//    pub addr: Recipient<ChatMessage>,
//}
//
//#[derive(Message)]
//#[rtype(result = "()")]
//struct Disconnect {
//    pub id: usize,
//}
//
