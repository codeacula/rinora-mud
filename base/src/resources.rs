use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use shared::prelude::*;

use crate::events::*;
use crate::models::*;

#[derive(Resource)]
pub struct NetworkInfo {
    pub connection_to_entity: HashMap<Uuid, Entity>,
}

#[derive(Resource)]
pub struct OutgoingQueue(pub Vec<OutgoingEvent>);

impl OutgoingQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn send_gmcp(&mut self, id: Uuid, gmcp: Vec<u8>) {
        self.0.push(OutgoingEvent {
            id,
            text: None,
            gmcp: Some(gmcp),
        });
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

impl Default for OutgoingQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OutgoingData(pub Sender<OutgoingEvent>);

pub struct NewConnectionListener(pub Receiver<NetworkEvent>);
