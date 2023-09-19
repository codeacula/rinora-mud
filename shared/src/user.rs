use bevy::{prelude::*, utils::Uuid};

#[derive(PartialEq, Default)]
pub enum UserStatus {
    CreateCharacter,
    CreatePassword,
    ConfirmPassword,
    DeleteCharacter,
    InGame,
    LoggedIn,
    #[default]
    NeedUsername,
    NeedPassword,
    ToggleAutologin,
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
