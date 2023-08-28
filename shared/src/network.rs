use std::collections::HashMap;

use bevy::{
    prelude::{Entity, Event, Resource},
    utils::Uuid,
};

#[derive(Event)]
pub struct NewConnectionEvent {
    pub entity: Entity,
    pub id: Uuid,
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

#[derive(Event)]
pub struct OutgoingEvent {
    pub id: Uuid,
    pub text: Option<Vec<u8>>,
    pub gmcp: Option<Vec<u8>>,
}

#[derive(Resource)]
pub struct OutgoingQueue(pub Vec<OutgoingEvent>);

impl OutgoingQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn send_text(&mut self, id: Uuid, text: String) {
        self.0.push(OutgoingEvent {
            id,
            text: Some(text.as_bytes().to_vec()),
            gmcp: None,
        });
    }

    pub fn send_str(&mut self, id: Uuid, text: &str) {
        self.send_text(id, text.to_string())
    }
}
