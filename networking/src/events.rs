use shared::prelude::*;

#[derive(Debug, Event)]
pub(crate) struct UserConnectedEvent(pub(crate) Uuid);

#[derive(Debug, Event)]
pub(crate) struct UserDisconnectedEvent(pub(crate) Uuid);

#[derive(Debug, Event)]
pub struct UserProvidedCommandEvent {
    pub id: Uuid,
    pub command: String,
}

#[derive(Debug, Event)]
pub struct UserProvidedGmcpEvent {
    pub id: Uuid,
    pub command: String,
    pub data: String,
}
