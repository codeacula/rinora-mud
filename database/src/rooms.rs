use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Collection,
    sync::Database,
};
use serde::{Deserialize, Serialize};
use shared::prelude::*;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DbExit {
    pub id: String,
    pub direction: String,
    pub to_room: ObjectId,
}

impl DbExit {
    pub fn to_game_exit(&self) -> Exit {
        Exit {
            id: self.id.clone(),
            direction: self.direction.clone(),
            to_room: self.to_room.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbRoom {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub description: String,

    pub can_delete: bool,

    pub exits: Vec<DbExit>,
}

impl Default for DbRoom {
    fn default() -> Self {
        DbRoom {
            id: None,
            name: Default::default(),
            description: Default::default(),
            can_delete: false,
            exits: Vec::new(),
        }
    }
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
