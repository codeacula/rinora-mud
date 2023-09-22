use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Collection,
    sync::Database,
};
use serde::{Deserialize, Serialize};
use shared::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbExit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub direction: String,
    pub to_room: ObjectId,
}

impl DbExit {
    pub fn to_game_exit(&self) -> Exit {
        Exit {
            id: self.id.unwrap().to_string(),
            direction: self.direction.clone(),
            to_room: self.to_room.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbRoom {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub description: String,

    pub exits: Vec<DbExit>,
}

impl DbRoom {
    pub fn to_game_room(&self) -> Room {
        Room {
            id: self.id.unwrap().to_string(),

            description: self.description.clone(),
            name: self.name.clone(),

            exits: self.exits.iter().map(|exit| exit.to_game_exit()).collect(),
        }
    }
}

pub struct RoomRepo {
    pub rooms: Collection<DbRoom>,
}

impl RoomRepo {
    pub fn new(database: &Database) -> Self {
        let rooms = database.collection::<DbRoom>("rooms");

        RoomRepo { rooms }
    }
}
