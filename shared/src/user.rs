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

#[derive(Event)]
pub struct AccountEvent {
    pub entity: Entity,
    pub input: Vec<String>,
    pub raw_command: String,
}

impl AccountEvent {
    pub fn get_input(&self) -> &Vec<String> {
        &self.input
    }

    pub fn get_keyword(&self) -> String {
        self.input[0].clone()
    }
}

#[derive(Component)]
pub struct User {
    pub autologin: String,
    pub dbid: String,
    pub username: String,
}

#[derive(Component)]
pub struct UserSessionData {
    pub connection: Uuid,
    pub pwd: Option<String>,
    pub status: UserStatus,
    pub username: String,
}
