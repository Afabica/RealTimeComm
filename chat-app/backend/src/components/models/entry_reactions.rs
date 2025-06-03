use serde::{Deserialize, Serialize};
use bson::{oid::ObjectId, DateTime};
use actix_web::{web, HttpResponse, Responder};
use mongodb::{ 
    options::ClientOptions,
    Client, Collection,
};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub user_id: Uuid,
    pub emoji: String,
    pub reacted_at: DateTime,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MemberEntry {
    pub user_id: Uuid,
    pub joined_at: DateTime,
    pub is_admin: bool,
}

impl MemberEntry {
    pub fn newMemberEntry(user_id: Uuid, joined_at: DateTime, is_admin: bool) -> Self {
        Self {
            user_id: user_id,
            joined_at: joined_at,
            is_admin: is_admin,
        }
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ArrayOfEmojis {
    pub entities: Vec<Reaction>,
    pub amount: Vec<i32>,
}

impl ArrayOfEmojis {
    pub fn newArrayOfEmojis(entities: Vec<Reaction>, amount: Vec<i32>) -> Self { 
        Self {
            entities: entities,
            amount: amount,
        }
    }

//    pub fn pushToEmojis(entities: Vec<Reaction>, entity: Reaction) {
//        for i in entities {
//            Reaction ent = entities[i];
//            if ent.emoji == entity.emoji {
//                 
//            }
//        }
//    }
}
    


