use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub id: i32,
    pub current_room_id: i32,
    pub shortname: String,
    pub user_id: i32,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            id: 0,
            shortname: "".to_string(),
            user_id: 0,
            current_room_id: 0,
        }
    }
}

#[derive(Event)]
pub struct CharacterEnteredWorld {
    pub character_id: i32,
}
