use bevy::{prelude::*, utils::Uuid};

#[derive(Component, Debug)]
pub struct LogOutUser {}

#[derive(Component, Debug)]
pub struct NeedsUsername {}

#[derive(Component, Debug)]
pub struct User {
    pub id: i32,
    pub administrator: bool,
    pub username: String,
}

#[derive(Component, Debug, Clone)]
pub struct UserSessionData {
    pub entity_they_are_controlling: Option<Entity>,
    pub connection: Uuid,
    pub username: Option<String>,
}

/*
#[derive(Event)]
pub struct UserLoggedInEvent {
    pub entity: Entity,
    pub id: i32,
    pub password: String,
}

#[derive(Eq, PartialEq, Default, Hash, Debug, Clone, Copy)]
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

impl UserSessionData {
    pub fn new() -> Self {
        Self {
            entity_they_are_controlling: None,
            connection: Uuid::nil(),
        }
    }
}

impl Default for UserSessionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Event)]
pub struct SendGmcpData {
    pub command_name: String,
    pub data: String,
    pub entity: Entity,
}
 */
