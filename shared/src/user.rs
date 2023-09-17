use bevy::{prelude::*, utils::Uuid};

#[derive(PartialEq)]
pub enum UserStatus {
    CreatePassword,
    ConfirmPassword,
    NeedUsername,
    NeedPassword,
    LoggedIn,
    InGame,
}

#[derive(Component)]
pub struct User {
    pub autologin: String,
    pub id: String,
    pub username: String,
}

#[derive(Component)]
pub struct UserSessionData {
    pub connection: Uuid,
    pub pwd: Option<String>,
    pub status: UserStatus,
    pub username: String,
}
