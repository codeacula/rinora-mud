use bevy::{ecs::system::Command, prelude::*};

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

pub struct DeleteCharacter {
    pub name: String,
}

#[derive(Event)]
pub struct DeleteCharacterEvent {
    pub name: String,
}

impl Command for DeleteCharacter {
    fn apply(self, world: &mut World) {
        world.send_event(DeleteCharacterEvent { name: self.name });
    }
}

#[derive(Event)]
pub struct CharacterEnteredWorld {
    pub character_id: i32,
}
