use bevy::{prelude::*, utils::Uuid};

#[derive(Event)]
pub struct UserLoggedInEvent {
    pub entity: Entity,
    pub id: i32,
    pub password: String,
}

#[derive(Eq, PartialEq, Default, Hash, Debug)]
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

#[derive(Component, Debug)]
pub struct User {
    pub id: i32,
    pub administrator: bool,
    pub username: String,
    pub current_character: Option<Entity>,
}

#[derive(Component, Debug)]
pub struct UserSessionData {
    pub controlling_entity: Option<Entity>,
    pub char_to_delete: Option<String>,
    pub connection: Uuid,
    pub pwd: Option<String>,
    pub status: UserStatus,
    pub username: String,
}

#[derive(Event)]
pub struct ShowPromptEvent(pub Entity);
