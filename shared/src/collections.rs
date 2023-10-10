use bevy::{prelude::*, utils::HashMap};

/**
 * These hash maps map entities to their respective IDs, so it's easier to look them up
 */
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
