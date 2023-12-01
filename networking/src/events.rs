use shared::prelude::*;

#[derive(Debug, Event)]
pub(crate) struct UserConnectedEvent(pub(crate) Uuid);

#[derive(Debug, Event)]
pub(crate) struct UserDisconnectedEvent(pub(crate) Uuid);

#[derive(Debug, Event)]
pub(crate) struct UserProvidedCommandEvent {
    pub(crate) id: Uuid,
    pub(crate) command: String,
}

#[derive(Debug, Event)]
pub(crate) struct UserProvidedGmcpEvent {
    pub(crate) id: Uuid,
    pub(crate) command: String,
    pub(crate) data: String,
}
