use crate::prelude::*;

#[derive(Debug, Event)]
pub struct GenericErrorEvent(pub Entity);

#[derive(Debug, Event)]
pub struct InvalidDirectionEvent(pub Entity);

#[derive(Debug, Event)]
pub struct PasswordsDoNotMatchEvent(pub Entity);

#[derive(Debug, Event)]
pub struct PleaseConfirmPasswordEvent(pub Entity);

#[derive(Debug, Event)]
pub struct ProvideUsernameEvent(pub Entity);

#[derive(Debug, Event)]
pub struct UserAccountCreatedEvent(pub Entity);
