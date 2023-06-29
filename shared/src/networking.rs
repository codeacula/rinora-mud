use std::sync::mpsc::Receiver;

use bevy::prelude::*;

pub struct NewConnectionListener(pub Receiver<NetworkEvent>);

/// A network
pub struct NetworkEvent {
    pub id: u64,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

/// What type of events the server will issue the game
pub enum NetworkEventType {
    NewConnection,
    InputReceived,
    ConnectionDropped,
    GmcpReceived,
}

pub struct OutgoingEvent {
    pub id: u64,
    pub text: Option<Vec<u8>>,
    pub gmcp: Option<Vec<u8>>,
}

#[derive(Resource)]
pub struct OutgoingQueue(pub Vec<OutgoingEvent>);
