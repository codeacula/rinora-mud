use bevy::{prelude::Event, utils::Uuid};

#[derive(Event)]
pub struct NewConnectionEvent(pub Uuid);

#[derive(Event)]
pub struct DisconnectionEvent(pub Uuid);

#[derive(Event)]
pub struct InputReceivedEvent {
    pub connection: Uuid,
    pub input: String,
}

#[derive(Event)]
pub struct GmcpReceivedEvent {
    pub connection: Uuid,
    pub data: Vec<u8>,
}
