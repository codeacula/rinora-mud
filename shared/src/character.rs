use bevy::{ecs::system::Command, prelude::*};

#[derive(Component)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub scheduled_for_deletion: bool,
    pub user_id: String,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            id: "".to_string(),
            name: "".to_string(),
            scheduled_for_deletion: false,
            user_id: "".to_string(),
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
