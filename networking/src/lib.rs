use std::net::TcpStream;

use network_functions::start_server::start_server;
use shared::prelude::*;
use systems::process_incoming_requests::*;

mod constants;
mod events;
mod network_functions;
mod stream_processor;
mod systems;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// What type of events the server will issue the game
pub enum NetworkEventType {
    Connect,
    Disconnect,
    Text,
    Gmcp,
}

/// Holds everything we need to identify a network connection
#[derive(Debug)]
pub struct NetworkConnection {
    pub id: Uuid,        // We use a UUID so we don't have to worry about integer rollover
    pub conn: TcpStream, // The TCP stream we use to communicate
    pub gmcp: bool,      // Whether or not the client has GMCP turned on
    pub do_room: bool,   // Whether or not we should send room data
}

#[derive(Debug, Clone)]
pub struct IncomingEvent {
    pub id: Uuid,
    pub command: Option<String>,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

#[derive(Debug, Clone)]
pub struct OutgoingEvent {
    pub id: Uuid,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let (outgoing_event_tx, incoming_event_rx) = start_server();

        app.insert_non_send_resource(outgoing_event_tx)
            .insert_non_send_resource(incoming_event_rx);

        app.add_systems(
            First,
            (process_incoming_requests).in_set(GameOrderSet::Network),
        );
    }
}
