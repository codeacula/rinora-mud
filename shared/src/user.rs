use bevy::prelude::Component;

pub enum UserStatus {
    Login,
    CharacterSelect,
    InGame,
}

#[derive(Component)]
pub struct User {
    pub connection: u64,
}
