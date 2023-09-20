use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader, ErrorKind, Write},
    net::TcpListener,
    net::TcpStream,
    sync::mpsc::*,
    thread,
};

use bevy::{prelude::*, utils::Uuid};
use outgoingtext::process_text_events_for_users;
use shared::prelude::*;

mod outgoingtext;

// All good MUDs have a banner!
const GREETING: &str = "
 _____  _                       __  __ _    _ _____  
|  __ \\(_)  Welcome to:        |  \\/  | |  | |  __ \\ 
| |__) |_ _ __   ___  _ __ __ _| \\  / | |  | | |  | |
|  _  /| | '_ \\ / _ \\| '__/ _` | |\\/| | |  | | |  | |
| | \\ \\| | | | | (_) | | | (_| | |  | | |__| | |__| |
|_|  \\_\\_|_| |_|\\___/|_|  \\__,_|_|  |_|\\____/|_____/ 


";

/// Main Bevy plugin to handle networking
pub struct NetworkServerPlugin;

/// Holds everything we need to identify a network connection
pub struct NetworkConnection {
    id: Uuid,        // We use a UUID so we don't have to worry about integer rollover
    conn: TcpStream, // The TCP stream we use to communicate
}

pub struct NewConnectionListener(pub Receiver<NetworkEvent>);

#[derive(Resource)]
pub struct NetworkInfo {
    pub connection_to_entity: HashMap<Uuid, Entity>,
}

pub struct NetworkEvent {
    pub id: Uuid,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

/// What type of events the server will issue the game
pub enum NetworkEventType {
    NewConnection,
    InputReceived,
    ConnectionDropped,
    GmcpReceived,
}

#[derive(Event)]
pub struct OutgoingEvent {
    pub id: Uuid,
    pub text: Option<Vec<u8>>,
    pub gmcp: Option<Vec<u8>>,
}

#[derive(Resource)]
pub struct OutgoingQueue(pub Vec<OutgoingEvent>);

impl OutgoingQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn send_text(&mut self, id: Uuid, text: String) {
        self.0.push(OutgoingEvent {
            id,
            text: Some(text.as_bytes().to_vec()),
            gmcp: None,
        });
    }

    pub fn send_str(&mut self, id: Uuid, text: &str) {
        self.send_text(id, text.to_string())
    }
}

impl Default for OutgoingQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OutgoingData(Sender<OutgoingEvent>);

fn check_for_new_connections(recv: &Receiver<NetworkConnection>) -> Vec<NetworkConnection> {
    let mut new_connections = Vec::<NetworkConnection>::new();

    loop {
        match recv.try_recv() {
            Ok(conn) => {
                debug!("New connection received from listener thread: {}", conn.id);
                new_connections.push(conn);
            }
            Err(err) => {
                if err == TryRecvError::Empty {
                    break;
                }

                warn!("Error communicating between threads: {}", err);
                break;
            }
        }
    }

    new_connections
}

fn start_listening(world: &mut World) {
    let (connection_event_tx, connection_event_rx) = channel::<NetworkEvent>();
    let (between_threads_tx, between_threads_rx) = channel::<NetworkConnection>();
    let (outgoing_event_tx, outgoing_event_rx) = channel::<OutgoingEvent>();

    // Main thread for listening to new connections
    thread::spawn(move || {
        // This is put into a separate thread because it blocks on the listener, and we don't want that to block
        // listening to the currently connected clients. I don't want to make the listener non-blocking because I don't
        // want to write error handling for that.

        let server_host = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
        let server_port = env::var("SERVER_PORT").unwrap_or(String::from("23"));

        let tcp_listener = TcpListener::bind(format!("{}:{}", server_host, server_port))
            .expect("Error starting TCP listener");

        for connection_result in tcp_listener.incoming() {
            let mut connection = match connection_result {
                Ok(conn) => conn,
                Err(err) => {
                    error!("Error accepting connection: {}", err);
                    break;
                }
            };

            if let Err(err) = connection.set_nonblocking(true) {
                error!("Failed to set to non-blocking: {}", err);
                break;
            }

            if let Err(err) = connection.write_all(GREETING.as_bytes()) {
                error!("Failed to write greeting, closing connection: {}", err);
                break;
            };

            if let Err(err) = between_threads_tx.send(NetworkConnection {
                id: Uuid::new_v4(),
                conn: connection,
            }) {
                error!("Failed to send connection to managing thread: {}", err);
                break;
            };
        }
    });

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

                let outgoing_text = match outgoing_event.text {
                    Some(text) => text,
                    None => break,
                };

                let write_res = outgoing_event_connection.conn.write_all(&outgoing_text);

                if write_res.is_ok() {
                    continue;
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
                        let mut reader = BufReader::new(&network_connection.conn);
                        let mut line = String::new();

                        if let Err(err) = reader.read_line(&mut line) {
                            warn!("Error reading line: {}", err);
                            continue;
                        }

                        if let Err(err) = connection_event_tx.send(NetworkEvent {
                            data: Some(line.into_bytes()),
                            id: network_connection.id,
                            event_type: NetworkEventType::InputReceived,
                        }) {
                            warn!("Failed to send network event to junction: {}", err);
                            continue;
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

fn process_outgoing_data(
    outgoing_data_rx: NonSend<OutgoingData>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    for event in outgoing_queue.0.drain(..) {
        if let Err(err) = outgoing_data_rx.0.send(event) {
            warn!("Failed to send outgoing event: {}", err);
        }
    }
}

/// Handles transferring new connections into the game world, and sending data from the game world to the client
fn transfer_from_server_to_game(
    connection_event_rx: NonSend<NewConnectionListener>,
    mut ev_new_connection: EventWriter<NewConnectionEvent>,
    mut ev_dropped_connection: EventWriter<DisconnectionEvent>,
    mut ev_input_received_connection: EventWriter<InputReceivedEvent>,
    mut ev_gmcp_received_connection: EventWriter<GmcpReceivedEvent>,
    mut network_info: ResMut<NetworkInfo>,
    mut commands: Commands,
) {
    while let Ok(new_event) = connection_event_rx.0.try_recv() {
        match new_event.event_type {
            NetworkEventType::NewConnection => {
                let entity = commands
                    .spawn((UserSessionData {
                        connection: new_event.id,
                        pwd: None,
                        status: UserStatus::NeedUsername,
                        username: String::new(),
                        char_to_delete: None,
                    },))
                    .id();
                network_info
                    .connection_to_entity
                    .insert(new_event.id, entity);
                ev_new_connection.send(NewConnectionEvent {
                    entity,
                    id: new_event.id,
                });
            }
            NetworkEventType::InputReceived => {
                let entity = *network_info
                    .connection_to_entity
                    .get(&new_event.id)
                    .unwrap();

                let input = String::from_utf8(new_event.data.unwrap()).unwrap();

                // We don't want to bother processing empty requests
                if input.trim().is_empty() {
                    continue;
                }

                ev_input_received_connection.send(InputReceivedEvent { entity, input });
            }
            NetworkEventType::ConnectionDropped => {
                let entity = network_info
                    .connection_to_entity
                    .remove(&new_event.id)
                    .unwrap();
                ev_dropped_connection.send(DisconnectionEvent { entity });
            }
            NetworkEventType::GmcpReceived => {
                let entity = *network_info
                    .connection_to_entity
                    .get(&new_event.id)
                    .unwrap();
                ev_gmcp_received_connection.send(GmcpReceivedEvent {
                    entity,
                    data: new_event.data.unwrap(),
                });
            }
        }
    }
}

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {
        let connection_hashmap = HashMap::<Uuid, Entity>::new();

        app.insert_resource(NetworkInfo {
            connection_to_entity: connection_hashmap,
        })
        .add_event::<NewConnectionEvent>()
        .add_event::<InputReceivedEvent>()
        .add_event::<DisconnectionEvent>()
        .add_event::<GmcpReceivedEvent>()
        .add_systems(Startup, start_listening)
        .add_systems(First, transfer_from_server_to_game)
        .add_systems(Last, (process_text_events_for_users, process_outgoing_data));
    }
}
