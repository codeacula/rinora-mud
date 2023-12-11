use shared::prelude::*;

#[derive(Debug, Event)]
pub(crate) struct UserConnectedEvent(pub(crate) Uuid);

#[derive(Debug, Event)]
pub(crate) struct UserDisconnectedEvent(pub(crate) Uuid);
