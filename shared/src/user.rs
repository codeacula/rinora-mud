use bevy::{prelude::*, utils::Uuid};

use crate::prelude::SentCommand;

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
    pub autologin: String,
    pub id: String,
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

pub struct UsernameProvided {}

// Events

#[derive(Event)]
pub struct UserProvidedUsername {
    pub command: SentCommand,
}

#[derive(Event)]
pub struct UserProvidedPassword {
    pub command: SentCommand,
}

#[derive(Event)]
pub struct UserCreatedPassword {
    pub command: SentCommand,
}

#[derive(Event)]
pub struct UserConfirmedPassword {
    pub command: SentCommand,
}

#[derive(Event)]
pub struct UserSelectedLoginOption {
    pub command: SentCommand,
}

#[derive(Event)]
pub struct UserProvidedCharacterName {
    pub command: SentCommand,
}

#[derive(Event)]
pub struct UserProvidedCharacterToDelete {
    pub command: SentCommand,
}

#[derive(Event)]
pub struct UserConfirmedDeleteCharacter {
    pub command: SentCommand,
}
