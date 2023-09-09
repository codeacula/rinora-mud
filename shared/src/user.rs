use bevy::{prelude::Component, utils::Uuid};

pub enum UserStatus {
    NeedUsername,
    NeedPassword,
    LoggedIn,
}

#[derive(Component)]
pub struct Login;

#[derive(Component)]
pub struct User {
    pub connection: Uuid,
    pub status: UserStatus,
    pub username: String,
}
