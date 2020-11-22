use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UID = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: u64,
    pub players: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRoom;

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinRoom {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetUsername {
    pub name: String,
}
