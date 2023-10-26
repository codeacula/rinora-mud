use bevy::prelude::*;

/// Fired when a character is created. Wraps the user entity
#[derive(Event)]
pub struct CharacterCreatedEvent(pub Entity);

/// Fired when a user tries to create a character that exists. Wraps the user entity
#[derive(Event)]
pub struct CharacterExistsEvent(pub Entity);

/// Fired when the user tries to provide an invalid user name. Wraps the user entity
#[derive(Event)]
pub struct CharacterNameInvalidEvent(pub Entity);

/// Fired when a user tries to select a character that doesn't exist on their account. Wraps the user entity
#[derive(Event)]
pub struct CharacterNotFoundEvent(pub Entity);

/// Fired when a user has selected a character from the main manu
#[derive(Event)]
pub struct CharacterSelectedEvent {
    pub name: String,
    pub user_entity: Entity,
}

/// Fired when a use tries to provide their confirmation password butx`` get it wrong
#[derive(Event)]
pub struct ConfirmPasswordDoesntMatch(pub Entity);

/// Fired when a user is ready to create a character through the control panel
#[derive(Event)]
pub struct CreateCharacterEvent {
    pub name: String,
    pub user_entity: Entity,
}

/// Fired when we want to show the user the login screen
#[derive(Event)]
pub struct ShowLoginScreenEvent(pub Entity);

/// Fired when the user is confirming their password and it's successful
#[derive(Event)]
pub struct UserConfirmedPassword(pub Entity);

/// When the user provides a username that isn't alphabetical
#[derive(Event)]
pub struct UsernameInvalid(pub Entity);
