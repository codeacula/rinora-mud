use bevy::{prelude::*, utils::Uuid};

use crate::prelude::UserCommand;

#[derive(Event)]
pub struct AccountEvent {
    pub entity: Entity,
    pub command: UserCommand,
}

#[derive(Event)]
pub struct UserLoggedIn {
    pub entity: Entity,
    pub id: i32,
}

#[derive(PartialEq, Default)]
pub enum UserStatus {
    CreateCharacter,
    CreatePassword,
    ConfirmDelete,
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
    pub id: i32,
    pub administrator: bool,
    pub autologin: Option<i32>,
    pub username: String,
}

#[derive(Component)]
pub struct UserSessionData {
    pub char_to_delete: Option<String>,
    pub connection: Uuid,
    pub pwd: Option<String>,
    pub status: UserStatus,
    pub username: String,
}
