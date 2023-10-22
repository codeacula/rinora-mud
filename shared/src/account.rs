use bevy::prelude::*;

/// Fired when a character is created. Wraps the user entity
#[derive(Event)]
pub struct CharacterCreatedEvent(pub Entity);

/// Fired when a user tries to create a character that exists. Wraps the user entity
#[derive(Event)]
pub struct CharacterExists(pub Entity);

/// Fired when the user tries to provide an invalid user name. Wraps the user entity
#[derive(Event)]
pub struct CharacterNameInvalid(pub Entity);

#[derive(Event)]
pub struct ShowLoginScreen(pub Entity);
