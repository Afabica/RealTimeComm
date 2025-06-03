//use serde::{Deserialize, Serialize};
//use mongodb::{bson::doc, error::Error, options::{ClientOptions, ResolverConfig}, Client};
//use mongodb::bson::{oid::ObjectId, DateTime};
//use dotenv:dotenv;
//use sqlx::PgPool;
//use tungestenite::stream::Mode;
//use chrono::Utc;
//
//
//#[derive(Debug, Serialize, Deserialize)]
//pub struct ApplicationMode {
//    #[serde(rename = "_id", skip_serializing_id = "Option::is_none")]
//    pub id:  Option<ObjectId>,
//    pub current_mode: Mode,
//    pub updated_at: DateTime,
//}
//
//impl ApplicationMode {
//    pub fn new(mode: Mode) -> Self {
//        Self {
//            id: None,
//            current_mode: mode,
//            updated_at: Utc::now().into(),
//        }
//    }
//}
