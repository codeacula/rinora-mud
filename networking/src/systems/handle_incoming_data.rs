use shared::prelude::*;
use std::{
    io::{ErrorKind, Read},
    net::TcpStream,
    sync::mpsc::Sender,
};

use crate::{stream_processor::*, IncomingEvent, NetworkConnection, NetworkEventType};

fn get_connection_data(mut conn: &TcpStream) -> Vec<u8> {
    let mut result = Vec::<u8>::new();

    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let amount_read = match conn.read(&mut buf) {
            Ok(amount) => amount,
            Err(e) if e.kind() == ErrorKind::WouldBlock => 0,
            Err(e) => panic!("Unexpected error: {}", e),
        };

        if amount_read == 0 {
            break;
        }
        result.extend_from_slice(&buf[..amount_read]);
    }

    result
}

pub(crate) fn handle_incoming_data(
    network_connection: &mut NetworkConnection,
    incoming_event_tx: &Sender<IncomingEvent>,
) {
    let user_provided_data = get_connection_data(&network_connection.conn);

    let mut buffer_process = BufferProcessor::new();

    for byte in user_provided_data {
        let command = buffer_process.next(byte);

        if let Some(command) = command {
            match command.command_type {
                NetworkCommandType::TurnOnGmcp => {
                    network_connection.gmcp = true;
                    info!("GMCP enabled for {}", network_connection.id);
                }
                NetworkCommandType::UserCommand => {
                    let data = &command.data.unwrap();
                    let line = String::from_utf8_lossy(data);

                    if let Err(err) = incoming_event_tx.send(IncomingEvent {
                        data: Some(line.as_bytes().to_vec()),
                        id: network_connection.id,
                        event_type: NetworkEventType::Text,
                    }) {
                        warn!("Failed to send network event to junction: {}", err);
                        continue;
                    }
                }
                NetworkCommandType::GmcpCommand => {
                    let name = command.command_name.clone();

                    if name == "Core.Supports.Set" {
                        let data = &command.data.unwrap();
                        let line = String::from_utf8_lossy(data);

                        if line.contains("Room 1") {
                            network_connection.do_room = true;
                        }
                    } else {
                        if let Err(err) = incoming_event_tx.send(IncomingEvent {
                            data: Some(command.data.unwrap()),
                            id: network_connection.id,
                            event_type: NetworkEventType::Gmcp,
                        }) {
                            warn!("Failed to send network event to junction: {}", err);
                            continue;
                        }
                    }
                    continue;
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, net::TcpListener};

    use crate::constants::*;

    use super::*;

    #[test]
    fn it_turns_on_gmcp() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let (inc_send, _inc_recv) = get_channels::<IncomingEvent>();

        let mut test_connection = NetworkConnection {
            id: Uuid::new_v4(),
            conn: TcpStream::connect(addr).unwrap(),
            gmcp: false,
            do_room: false,
        };

        // Tell the server to turn on GMCP
        test_connection.conn.write_all(&[IAC, DO, GMCP]).unwrap();

        handle_incoming_data(&mut test_connection, &inc_send);
        assert!(test_connection.gmcp);
        drop(listener);
    }
}
