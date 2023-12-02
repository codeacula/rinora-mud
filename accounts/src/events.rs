use shared::prelude::*;

#[derive(Debug, Event)]
pub(crate) struct InvalidUsernameFormatEvent(pub(crate) Entity);

#[derive(Debug, Event)]
pub(crate) struct CreatingNewAccountEvent(pub(crate) Entity);

#[derive(Debug, Event)]
pub(crate) struct ConfirmingPasswordEvent(pub(crate) Entity);
