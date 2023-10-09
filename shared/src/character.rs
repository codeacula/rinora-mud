use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Character {
    pub id: i32,
    pub shortname: String,
    pub user_id: i32,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub info: Character,
    pub location: Location,
    pub health: Health,
    pub mana: Mana,
}

#[derive(Component)]
pub struct IsControlledBy(Entity);

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            health: Health { current: 0, max: 0 },
            mana: Mana { current: 0, max: 0 },
            location: Location(0),
            info: Character {
                id: 0,
                shortname: "".to_string(),
                user_id: 0,
            },
        }
    }
}

#[derive(Resource)]
pub struct CharacterMap(pub HashMap<i32, Entity>);
