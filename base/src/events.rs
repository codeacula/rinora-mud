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

#[derive(Event, Clone)]
pub struct ShowRoomToBeing {
    pub entity: Entity,
    pub room: Entity,
}
