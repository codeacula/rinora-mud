use bevy::prelude::*;

#[derive(Component)]
pub struct Room {
    pub id: String,

    pub name: String,
    pub description: String,

    pub exits: Vec<Exit>,
}

pub struct Exit {
    pub id: String,

    pub direction: String,
    pub to_room: String,
}
