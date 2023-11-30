use std::{
    io::ErrorKind,
    sync::mpsc::{Receiver, Sender},
};

use shared::prelude::*;

use crate::*;

use super::{
    add_new_connections::*, handle_incoming_data::handle_incoming_data, process_outgoing_events::*,
    shutdown_connection::*,
};

pub(crate) fn process_connections(
    between_threads_rx: Receiver<NetworkConnection>,
    outgoing_event_rx: Receiver<OutgoingEvent>,
    incoming_event_tx: Sender<IncomingEvent>,
) {
    let mut all_connections = Vec::<NetworkConnection>::new();

    loop {
        let mut to_remove = Vec::<Uuid>::new();

        add_new_connections(
            &mut all_connections,
            &between_threads_rx,
            &incoming_event_tx,
        );

        process_outgoing_events(&mut all_connections, &outgoing_event_rx, &incoming_event_tx);

        for network_connection in &mut all_connections {
            let mut buf = [0; 1024];

            match network_connection.conn.peek(&mut buf) {
                Ok(0) => {
                    // Connection closed
                    to_remove.push(network_connection.id);
                    continue;
                }
                Ok(_) => {
                    handle_incoming_data(network_connection, &incoming_event_tx);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    // No data available yet
                    continue;
                }
                Err(e) => panic!("Unexpected error: {}", e),
            }
        }

        for id in to_remove {
            shutdown_connection(id, &mut all_connections, &incoming_event_tx);
        }
    }
}
