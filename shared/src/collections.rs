use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct CharacterMap(pub HashMap<i32, Entity>);

#[derive(Resource)]
pub struct PlaneMap(pub HashMap<i32, Entity>);

#[derive(Resource)]
pub struct ContinentMap(pub HashMap<i32, Entity>);

#[derive(Resource)]
pub struct AreaMap(pub HashMap<i32, Entity>);

#[derive(Resource)]
pub struct RoomMap(pub HashMap<i32, Entity>);
