use shared::prelude::*;

#[derive(Debug, Event)]
pub(crate) struct AccountNotFoundEvent(pub(crate) Entity);

#[derive(Debug, Event)]
pub(crate) struct CreatingNewAccountEvent(pub(crate) Entity);

#[derive(Debug, Event)]
pub(crate) struct InvalidUsernameFormatEvent(pub(crate) Entity);

#[derive(Debug, Event)]
pub(crate) struct LoggingInEvent(pub(crate) Entity);

#[derive(Debug, Event)]
pub(crate) struct WelcomeUserEvent(pub(crate) Entity);
