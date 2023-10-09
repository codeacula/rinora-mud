use bevy::{prelude::*, utils::Uuid};

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
    pub current_character: Option<Entity>,
}

#[derive(Component)]
pub struct UserSessionData {
    pub controlling_entity: Option<Entity>,
    pub char_to_delete: Option<String>,
    pub connection: Uuid,
    pub pwd: Option<String>,
    pub status: UserStatus,
    pub username: String,
}
