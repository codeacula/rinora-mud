use crate::prelude::*;

#[derive(Event)]
pub struct GenericErrorEvent(pub Entity);

#[derive(Event)]
pub struct InvalidDirectionEvent(pub Entity);

#[derive(Event)]
pub struct PleaseConfirmPassword(pub Entity);

#[derive(Debug, Event)]
pub struct ProvideUsernameEvent(pub Entity);

#[derive(Debug, Event)]
pub struct UserAccountCreatedEvent(pub Entity);
