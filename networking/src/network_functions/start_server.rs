use std::{sync::mpsc::*, thread};

use crate::{IncomingEvent, NetworkConnection, OutgoingEvent};

use super::{process_connections::*, start_listening::*};

pub fn start_server() -> (Sender<OutgoingEvent>, Receiver<IncomingEvent>) {
    let (incoming_event_tx, incoming_event_rx) = channel::<IncomingEvent>();
    let (between_threads_tx, between_threads_rx) = channel::<NetworkConnection>();
    let (outgoing_event_tx, outgoing_event_rx) = channel::<OutgoingEvent>();

    // Main thread for listening to new connections
    thread::spawn(move || start_listening(between_threads_tx));

    // Sends new connections to the game world, along with new commands or GMCP commands. Also disconnects.
    thread::spawn(move || {
        process_connections(between_threads_rx, outgoing_event_rx, incoming_event_tx)
    });

    (outgoing_event_tx, incoming_event_rx)
}
