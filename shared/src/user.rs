use bevy::{prelude::*, utils::Uuid};

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

#[derive(Component, Debug)]
pub struct User {
    pub id: i32,
    pub administrator: bool,
    pub username: String,
}

#[derive(Component, Debug, Clone)]
pub struct UserSessionData {
    pub controlling_entity: Option<Entity>,
    pub char_to_delete: Option<String>,
    pub connection: Uuid,
    pub pwd: Option<String>,
    pub status: UserStatus,
    pub username: String,
}

impl UserSessionData {
    pub fn new() -> Self {
        Self {
            controlling_entity: None,
            char_to_delete: None,
            connection: Uuid::nil(),
            pwd: None,
            status: UserStatus::NeedUsername,
            username: String::new(),
        }
    }
}

impl Default for UserSessionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Event)]
pub struct ShowPromptEvent(pub Entity);

#[derive(Debug, Clone, Event)]
pub struct SendGmcpData {
    pub command_name: String,
    pub data: String,
    pub entity: Entity,
}
