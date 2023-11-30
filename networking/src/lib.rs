use std::{net::TcpStream, sync::mpsc::*, thread};

use shared::prelude::*;
use systems::{
    process_connections::{self},
    start_listening::*,
};

mod constants;
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
pub struct NetworkConnection {
    pub id: Uuid,        // We use a UUID so we don't have to worry about integer rollover
    pub conn: TcpStream, // The TCP stream we use to communicate
    pub gmcp: bool,      // Whether or not the client has GMCP turned on
    pub do_room: bool,   // Whether or not we should send room data
}

#[derive(Debug, Clone)]
pub struct IncomingEvent {
    pub id: Uuid,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

pub struct OutgoingEvent {
    pub id: Uuid,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

pub fn start_server(world: &mut World) {
    let (connection_event_tx, connection_event_rx) = channel::<IncomingEvent>();
    let (between_threads_tx, between_threads_rx) = channel::<NetworkConnection>();
    let (outgoing_event_tx, outgoing_event_rx) = channel::<OutgoingEvent>();

    // Main thread for listening to new connections
    thread::spawn(move || start_listening(between_threads_tx));

    // Sends new connections to the game world, along with new commands or GMCP commands. Also disconnects.
    //thread::spawn(move || process_connections(between_threads_rx));
}
