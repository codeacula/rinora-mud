use bevy::{prelude::*, utils::HashMap};

use crate::prelude::*;

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

impl RoomMap {
    pub fn get_room(&self, location: &Location) -> Option<Entity> {
        let ent = self.0.get(&location.0);

        if ent.is_none() {
            return None;
        }

        Some(*ent.unwrap())
    }
}
