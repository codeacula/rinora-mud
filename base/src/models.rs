use shared::prelude::*;
use std::net::TcpStream;

use crate::enums::*;

/// Holds everything we need to identify a network connection
pub struct NetworkConnection {
    pub id: Uuid,         // We use a UUID so we don't have to worry about integer rollover
    pub conn: TcpStream,  // The TCP stream we use to communicate
    pub gmcp: bool,       // Whether or not the client has GMCP turned on
    pub send_room: bool,  // Does the user want room info via GMCP?
    pub send_chat: bool,  // Does the user want chat info via GMCP?
    pub send_stats: bool, // Should we send character stats via GMCP?
    pub send_time: bool, //Send time changes via GMCP. This should be morning, afternoon, dusk, night, and midnights
}

pub struct NetworkEvent {
    pub id: Uuid,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}
