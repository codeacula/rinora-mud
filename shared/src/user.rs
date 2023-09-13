use bevy::{prelude::*, utils::Uuid};

#[derive(PartialEq)]
pub enum UserStatus {
    NeedUsername,
    NeedPassword,
    LoggedIn,
    InGame,
}

#[derive(Event)]
pub struct AccountEvent {
    pub entity: Entity,
    pub input: Vec<String>,
}

#[derive(Component)]
pub struct User {
    pub connection: Uuid,
    pub status: UserStatus,
    pub username: String,
}
