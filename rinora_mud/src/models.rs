use shared::prelude::*;
use std::net::TcpStream;

use crate::enums::*;

/// Holds everything we need to identify a network connection
pub struct NetworkConnection {
    pub id: Uuid,        // We use a UUID so we don't have to worry about integer rollover
    pub conn: TcpStream, // The TCP stream we use to communicate
}

pub struct NetworkEvent {
    pub id: Uuid,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}
