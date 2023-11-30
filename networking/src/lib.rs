use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::*,
    thread,
};

use shared::prelude::*;
use stream_processor::BufferProcessor;
use systems::{check_for_new_connections::*, start_listening::*};

mod constants;
mod stream_processor;
mod systems;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// What type of events the server will issue the game
pub enum NetworkEventType {
    NewConnection,
    InputReceived,
    ConnectionDropped,
}

/// Holds everything we need to identify a network connection
pub struct NetworkConnection {
    pub id: Uuid,        // We use a UUID so we don't have to worry about integer rollover
    pub conn: TcpStream, // The TCP stream we use to communicate
    pub gmcp: bool,      // Whether or not the client has GMCP turned on
    pub do_room: bool,   // Whether or not we should send room data
}

#[derive(Debug, Clone)]
pub struct NetworkEvent {
    pub id: Uuid,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

/*
#[derive(Event)]
pub struct OutgoingEvent {
    pub id: Uuid,
    pub text: Option<Vec<u8>>,
    pub gmcp: Option<Vec<u8>>,
}

pub fn start_server(world: &mut World) {
    let (connection_event_tx, connection_event_rx) = channel::<NetworkEvent>();
    let (between_threads_tx, between_threads_rx) = channel::<NetworkConnection>();
    let (outgoing_event_tx, outgoing_event_rx) = channel::<OutgoingEvent>();

    // Main thread for listening to new connections
    thread::spawn(move || start_listening(between_threads_tx));

    // Sends new connections to the game world, along with new commands or GMCP commands. Also disconnects.
    thread::spawn(move || {
        let mut all_connections = Vec::<NetworkConnection>::new();

        loop {
            let mut to_remove = Vec::<Uuid>::new();

            let new_connections = check_for_new_connections(&between_threads_rx);

            for new_conn in new_connections {
                if let Err(err) = connection_event_tx.send(NetworkEvent {
                    id: new_conn.id,
                    data: None,
                    event_type: NetworkEventType::NewConnection,
                }) {
                    error!("Failed to send network event to junction: {}", err);
                    break;
                };
                all_connections.push(new_conn);
            }

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

                let outgoing_event_connection = match all_connections
                    .iter_mut()
                    .find(|conn| conn.id == outgoing_event.id)
                {
                    Some(conn) => conn,
                    None => break,
                };

                if outgoing_event.text.is_some() {
                    let outgoing_text = outgoing_event.text.unwrap();
                    let write_res = outgoing_event_connection.conn.write_all(&outgoing_text);

                    if write_res.is_ok()
                        && (!outgoing_event_connection.gmcp || outgoing_event.gmcp.is_none())
                    {
                        continue;
                    }
                }

                if outgoing_event.gmcp.is_some() {
                    let data = outgoing_event.gmcp.unwrap();
                    let gmcp_write_res = outgoing_event_connection.conn.write_all(&data);

                    if gmcp_write_res.is_ok() {
                        continue;
                    }
                }

                // Connection closed
                to_remove.push(outgoing_event_connection.id);

                if outgoing_event_connection
                    .conn
                    .shutdown(std::net::Shutdown::Both)
                    .is_err()
                {
                    warn!("Failed to shutdown connection");
                    break;
                }

                // Send the connection dropped event to the game because we can't write to them anymore
                if let Err(err) = connection_event_tx.send(NetworkEvent {
                    id: outgoing_event_connection.id,
                    data: None,
                    event_type: NetworkEventType::ConnectionDropped,
                }) {
                    error!("Failed to send connection dropped event: {:?}", err);
                    break;
                };

                continue;
            }

            for network_connection in &mut all_connections {
                let mut buf = [0; 1024];

                match network_connection.conn.peek(&mut buf) {
                    Ok(0) => {
                        // Connection closed
                        if network_connection
                            .conn
                            .shutdown(std::net::Shutdown::Both)
                            .is_err()
                        {
                            to_remove.push(network_connection.id);
                            warn!("Failed to shutdown connection, still discarding.");
                            continue;
                        }

                        to_remove.push(network_connection.id);

                        let send_success = connection_event_tx.send(NetworkEvent {
                            data: None,
                            id: network_connection.id,
                            event_type: NetworkEventType::ConnectionDropped,
                        });

                        if let Err(err) = send_success {
                            warn!("Failed to send connection dropped event: {}", err);
                        };

                        continue;
                    }
                    Ok(_) => {
                        let mut buffer: Vec<u8> = Vec::new();

                        loop {
                            let mut buf: [u8; 1024] = [0; 1024];
                            let amount_read = match network_connection.conn.read(&mut buf) {
                                Ok(amount) => amount,
                                Err(e) if e.kind() == ErrorKind::WouldBlock => 0,
                                Err(e) => panic!("Unexpected error: {}", e),
                            };

                            if amount_read == 0 {
                                break;
                            }
                            buffer.extend_from_slice(&buf[..amount_read]);
                        }

                        let mut buffer_process = BufferProcessor::new();

                        for byte in buffer {
                            let command = buffer_process.next(byte);

                            if let Some(command) = command {
                                match command.command_type {
                                    stream_processor::NetworkCommandType::TurnOnGmcp => {
                                        network_connection.gmcp = true;
                                        info!("GMCP enabled for {}", network_connection.id);
                                    }
                                    stream_processor::NetworkCommandType::UserCommand => {
                                        let data = &command.data.unwrap();
                                        let line = String::from_utf8_lossy(data);

                                        if let Err(err) = connection_event_tx.send(NetworkEvent {
                                            data: Some(line.as_bytes().to_vec()),
                                            id: network_connection.id,
                                            event_type: NetworkEventType::InputReceived,
                                        }) {
                                            warn!(
                                                "Failed to send network event to junction: {}",
                                                err
                                            );
                                            continue;
                                        }
                                    }
                                    stream_processor::NetworkCommandType::GmcpCommand => {
                                        let name = command.command_name.clone();

                                        if name == "Core.Supports.Set" {
                                            let data = &command.data.unwrap();
                                            let line = String::from_utf8_lossy(data);

                                            if line.contains("Room 1") {
                                                network_connection.do_room = true;
                                            }
                                        }
                                        continue;
                                    }
                                }
                            };
                        }
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        // No data available yet
                        continue;
                    }
                    Err(e) => panic!("Unexpected error: {}", e),
                }
            }

            for id in to_remove {
                all_connections.retain(|conn| conn.id != id);
            }
        }
    });

    world.insert_non_send_resource(NewConnectionListener(connection_event_rx));
    world.insert_non_send_resource(OutgoingData(outgoing_event_tx));
    world.insert_resource(OutgoingQueue(Vec::new()));
}
 */
