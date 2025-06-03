use crate::components::models::{chat_group::ChatGroup, messages::Message};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Database, Collection, error::Error, results::{InsertOneResult, UpdateResult}};
use actix_web::{web, Responder, HttpResponse};
use uuid::Uuid;

use futures::stream::TryStreamExt;


pub struct MongoRepo {
    pub db: Database,
}

impl MongoRepo {
    pub fn new(client: Client) -> Self {
        let db = client.database("chatapp");
        MongoRepo { db }
    }
//    pub async fn init(client: &Client) -> Database {
//        let client = Client::with_uri_str("mongodb://localhost:27017")
//            .await
//            .expect("Failed to initialize MongoDB client");
//
//        let db = client.database("chat_app");
//        db
//    }

    pub async fn create_group(&self, group: ChatGroup) -> Result<InsertOneResult, Error> {
        let collection: Collection<ChatGroup> = self.db.collection("groups");
        collection.insert_one(group, None).await 
    }

     pub async fn get_group_by_id(&self, id: ObjectId) -> Result<Option<ChatGroup>, Error> {
        let collection = self.db.collection::<ChatGroup>("groups");
        collection.find_one(doc! { "_id": id }, None).await
    }

     pub async fn add_member_to_group(&self, group_id: ObjectId, user_id: Uuid) -> Result<UpdateResult, Error> {
         let collection = self.db.collection::<ChatGroup>("groups");
         collection.update_one(
             doc! { "_id": group_id},
             doc! { "$addToSet": { "member_ids": user_id.to_string()}},
             None,
         ).await
     }
    
    pub async fn list_user_groups(&self, user_id: Uuid) -> Result<Vec<ChatGroup>, Error> {
        let collection = self.db.collection::<ChatGroup>("groups");
        let filter = doc! { "member_ids": user_id.to_string()};
        let mut cursor = collection.find(filter, None).await?;
        let mut groups = Vec::new();
        while let Some(group) = cursor.try_next().await? {
            groups.push(group);
        }
        Ok(groups)
    }
}
