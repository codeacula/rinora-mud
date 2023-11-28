use bevy::{prelude::*, utils::Uuid};

#[derive(Event)]
pub struct DisconnectionEvent {
    pub entity: Entity,
}

#[derive(Event, Clone)]
pub struct InputReceivedEvent {
    pub entity: Entity,
    pub input: String,
}

#[derive(Event, Clone)]
pub struct InvalidCommandEvent(pub Entity);

#[derive(Event)]
pub struct NewConnectionEvent {
    pub entity: Entity,
    pub id: Uuid,
}

#[derive(Event)]
pub struct OutgoingEvent {
    pub id: Uuid,
    pub text: Option<Vec<u8>>,
    pub gmcp: Option<Vec<u8>>,
}

#[derive(Event, Clone)]
pub struct ShowRoomToBeing {
    pub entity: Entity,
    pub room: Entity,
}
