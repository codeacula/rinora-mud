use bevy::{ecs::system::Command, prelude::*};

#[derive(Component)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub user_id: String,
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
        world.send_event(DeleteCharacterEvent { name: self.name })
    }
}
