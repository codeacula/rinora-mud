use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Plane {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub continents: Vec<Entity>,
}

#[derive(Component)]
pub struct Continent {
    pub id: i32,
    pub plane_id: i32,
    pub name: String,
    pub description: String,
    pub areas: Vec<Entity>,
}

#[derive(Component)]
pub struct Area {
    pub id: i32,
    pub continent_id: i32,
    pub name: String,
    pub description: String,
    pub rooms: Vec<Entity>,
}

#[derive(Component, Clone)]
pub struct Environment {
    pub id: i32,
    pub name: String,
}

#[derive(Component, Debug)]
pub struct Room {
    pub id: i32,
    pub area_id: i32,
    pub name: String,
    pub description: String,
    pub environment_id: i32,
    pub exits: Vec<Entity>,
    pub entities: Vec<Entity>,
}

#[derive(Component)]
pub struct Exit {
    pub id: i32,
    pub from_room_id: i32,
    pub to_room_id: i32,
    pub direction: String,
    pub hidden: bool,

    pub to_room: Entity,
}

#[derive(Component)]
pub struct Location(pub i32);

#[derive(Event, Debug)]
pub struct EntityEnteredRoom {
    pub entity: Entity,
    pub room: Entity,
}

#[derive(Event, Debug)]
pub struct EntityEnteredWorld {
    pub entity: Entity,
    pub room: Entity,
}

#[derive(Resource)]
pub struct PlaneMap(pub HashMap<i32, Entity>);

#[derive(Resource)]
pub struct ContinentMap(pub HashMap<i32, Entity>);

#[derive(Resource)]
pub struct AreaMap(pub HashMap<i32, Entity>);

#[derive(Resource)]
pub struct RoomMap(pub HashMap<i32, Entity>);
