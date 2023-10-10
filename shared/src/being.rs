use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Being {}

#[derive(Component)]
pub struct Character {
    pub id: i32,
    pub shortname: String,
    pub user_id: i32,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub being: Being,
    pub info: Character,
    pub location: Location,
    pub health: Health,
    pub mana: Mana,
}

#[derive(Component)]
pub struct IsControlledBy(pub Entity);

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            being: Being {},
            health: Health { current: 0, max: 0 },
            mana: Mana { current: 0, max: 0 },
            location: Location(1),
            info: Character {
                id: 0,
                shortname: "".to_string(),
                user_id: 0,
            },
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Mana {
    pub current: i32,
    pub max: i32,
}
