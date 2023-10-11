use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Being {}

#[derive(Component)]
pub struct Character {
    pub character_id: i32,
    pub user_id: i32,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub being: Being,
    pub description: Description,
    pub display_name: DisplayName,
    pub info: Character,
    pub location: Location,
    pub health: Health,
    pub mana: Mana,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            being: Being {},
            description: Description("".to_string()),
            display_name: DisplayName("".to_string()),
            health: Health { current: 0, max: 0 },
            mana: Mana { current: 0, max: 0 },
            location: Location(1),
            info: Character {
                character_id: 0,
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
pub struct IsControlledBy(pub Entity);

#[derive(Component)]
pub struct Mana {
    pub current: i32,
    pub max: i32,
}
