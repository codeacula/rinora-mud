use bevy::{ecs::system::Command, prelude::*, utils::Uuid};

use crate::prelude::SentCommand;

pub struct TransitionUserToState {
    pub entity: Entity,
    pub state: UserStatus,
}

impl Command for TransitionUserToState {
    fn apply(self, world: &mut World) {
        let Some(mut found_entity) = world.get_entity_mut(self.entity) else {
            error!("Unable to transition user state: Entity not found");
            return;
        };

        let Some(mut user) = found_entity.get_mut::<UserSessionData>() else {
            error!("Unable to transition user state: User not found");
            return;
        };

        user.status = self.state;
    }
}

#[derive(Event)]
pub struct AccountEvent {
    pub entity: Entity,
    pub command: SentCommand,
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
