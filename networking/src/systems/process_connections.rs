use std::sync::mpsc::Receiver;

use shared::prelude::*;

use crate::NetworkConnection;

use super::add_new_connections::*;

pub(crate) fn process_connections(
    between_threads_rx: Receiver<NetworkConnection>,
    outgoing_event_rx: Receiver<OutgoingData>,
    connection_event_tx: Sender<NetworkEvent>,
) {
    let mut all_connections = Vec::<NetworkConnection>::new();

    loop {
        let mut to_remove = Vec::<Uuid>::new();

        add_new_connections(&mut all_connections, between_threads_rx, outgoing_event_rx);

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
                                        warn!("Failed to send network event to junction: {}", err);
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
}
