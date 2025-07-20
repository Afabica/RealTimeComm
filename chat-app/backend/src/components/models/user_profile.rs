use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    pub id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

impl UserProfile {
    pub fn new(id: Uuid, username: String, email: String, roles: Vec<String>) -> Self {
        Self {
            id: Some(id),
            username,
            email,
            roles,
        }
    }
}

