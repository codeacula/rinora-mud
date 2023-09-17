use bevy::prelude::*;

#[derive(Event)]
pub struct AccountEvent {
    pub entity: Entity,
    pub input: Vec<String>,
    pub raw_command: String,
}

#[derive(Event)]
pub struct LoginOptionSelected {
    pub entity: Entity,
    pub option: String,
}

#[derive(Event)]
pub struct UserCreatedPassword {
    pub entity: Entity,
    pub password: String,
}

#[derive(Event)]
pub struct UserConfirmedPassword {
    pub entity: Entity,
    pub password: String,
}

#[derive(Event)]
pub struct UserLoggedIn {
    pub entity: Entity,
    pub uuid: String,
}

#[derive(Event)]
pub struct UserProvidedPassword {
    pub entity: Entity,
    pub password: String,
}

#[derive(Event)]
pub struct UserProvidedUsername {
    pub entity: Entity,
    pub username: String,
}
