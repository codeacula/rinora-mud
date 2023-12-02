use std::{
    io::Write,
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

use shared::prelude::*;

use crate::{IncomingEvent, NetworkConnection, NetworkEventType, OutgoingEvent};

use super::shutdown_connection::*;

pub(crate) fn process_outgoing_events(
    all_connections: &mut Vec<NetworkConnection>,
    outgoing_event_rx: &Receiver<OutgoingEvent>,
    incoming_event_tx: &Sender<IncomingEvent>,
) {
    loop {
        let outgoing_event = match outgoing_event_rx.try_recv() {
            Ok(event) => event,
            Err(err) => {
                if err == TryRecvError::Empty {
                    break;
                }

                warn!("Error receiving from outgoing event: {}", err);
                break;
            }
        };

        if outgoing_event.event_type == NetworkEventType::Disconnect {
            shutdown_connection(outgoing_event.id, all_connections, incoming_event_tx);
            continue;
        }

        let outgoing_event_connection = match all_connections
            .iter_mut()
            .find(|conn| conn.id == outgoing_event.id)
        {
            Some(conn) => conn,
            None => break,
        };

        if outgoing_event.data.is_some() {
            info!("Sending outgoing event: {:?}", outgoing_event);
            let write_res = outgoing_event_connection
                .conn
                .write_all(&outgoing_event.data.unwrap());

            if write_res.is_ok() {
                continue;
            }
        }

        if outgoing_event_connection
            .conn
            .shutdown(std::net::Shutdown::Both)
            .is_err()
        {
            warn!("Failed to shutdown connection");
            break;
        }

        continue;
    }
}
