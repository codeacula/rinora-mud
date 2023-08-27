use std::collections::HashMap;

use bevy::{
    prelude::{Entity, Event, Resource},
    utils::Uuid,
};

#[derive(Event)]
pub struct NewConnectionEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct DisconnectionEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct InputReceivedEvent {
    pub entity: Entity,
    pub input: String,
}

#[derive(Event)]
pub struct GmcpReceivedEvent {
    pub entity: Entity,
    pub data: Vec<u8>,
}

#[derive(Resource)]
pub struct NetworkInfo {
    pub connection_to_entity: HashMap<Uuid, Entity>,
}
