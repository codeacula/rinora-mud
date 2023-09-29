use bevy::{prelude::*, utils::HashMap};

pub struct Plane {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct Continent {
    pub id: i32,
    pub plane_id: i32,
    pub name: String,
    pub description: String,
}

pub struct Area {
    pub id: i32,
    pub continent_id: i32,
    pub name: String,
    pub description: String,
}

pub struct Environment {
    pub id: i32,
    pub name: String,
}

pub struct Room {
    pub id: i32,
    pub area_id: i32,
    pub name: String,
    pub description: String,
    pub environment_id: i32,
}

pub struct Exit {
    pub id: i32,
    pub from_room_id: i32,
    pub to_room_id: i32,
    pub direction: String,
    pub hidden: bool,
}

#[derive(Resource)]
pub struct GameWorld {
    pub planes: Vec<Plane>,
    pub continents: Vec<Continent>,
    pub areas: Vec<Area>,
    pub environments: Vec<Environment>,
    pub rooms: Vec<Room>,
    pub rooms_by_id: HashMap<i32, usize>,
    pub exits: Vec<Exit>,
}

impl GameWorld {
    pub fn get_room_by_id(&self, room_id: i32) -> Option<&Room> {
        let room_pos = self.rooms_by_id.get(&room_id);
        if room_pos.is_none() {
            return None;
        }

        Some(&self.rooms[*room_pos.unwrap()])
    }
}
