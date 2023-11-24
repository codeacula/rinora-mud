use crate::prelude::*;

#[derive(Event)]
pub struct GenericErrorEvent(pub Entity);

#[derive(Event)]
pub struct InvalidDirectionEvent(pub Entity);

#[derive(Event)]
pub struct PasswordsDoNotMatchEvent(pub Entity);

#[derive(Event)]
pub struct PleaseConfirmPasswordEvent(pub Entity);

#[derive(Debug, Event)]
pub struct ProvideUsernameEvent(pub Entity);

#[derive(Debug, Event)]
pub struct UserAccountCreatedEvent(pub Entity);
