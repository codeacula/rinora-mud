use std::{
    env,
    io::{BufRead, BufReader, ErrorKind, Write},
    net::TcpListener,
    net::TcpStream,
    sync::mpsc::*,
    thread,
};

use bevy::prelude::*;

// All good MUDs have a banner!
const GREETING: &str = "
 _____  _                       __  __ _    _ _____  
|  __ \\(_)  Welcome to:        |  \\/  | |  | |  __ \\ 
| |__) |_ _ __   ___  _ __ __ _| \\  / | |  | | |  | |
|  _  /| | '_ \\ / _ \\| '__/ _` | |\\/| | |  | | |  | |
| | \\ \\| | | | | (_) | | | (_| | |  | | |__| | |__| |
|_|  \\_\\_|_| |_|\\___/|_|  \\__,_|_|  |_|\\____/|_____/ 


";
pub struct NetworkServerPlugin;

pub struct NetworkConnection {
    id: u64,
    conn: TcpStream,
}

pub struct NewConnectionListener(pub Receiver<NetworkEvent>);

/// A network
pub struct NetworkEvent {
    pub id: u64,
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

pub struct OutgoingEvent {
    pub id: u64,
    pub text: Option<Vec<u8>>,
    pub gmcp: Option<Vec<u8>>,
}

pub struct NewConnectionEvent(u64);

#[derive(Resource)]
pub struct OutgoingQueue(pub Vec<OutgoingEvent>);

/*
/// Telnet protocol constants
/// Byte to signal subchannel negotiation
const IAC: i32 = 255;

/// Client WILL do something. Mostly GMCP
const WILL: i32 = 251;

/// Client WONT do something. Mostly GMCP
const WONT: i32 = 252;

/// Client requests server to DO something
const DO: i32 = 253;

/// Client requests server to DONT do something
const DONT: i32 = 254;

/// GMCP byte flag
const GMCP: i32 = 201;
*/

pub struct OutgoingData(Sender<OutgoingEvent>);

fn start_listening(world: &mut World) {
    let (connection_event_tx, connection_event_rx) = channel::<NetworkEvent>();
    let (between_threads_tx, between_threads_rx) = channel::<NetworkConnection>();
    let (outgoing_event_tx, outgoing_event_rx) = channel::<OutgoingEvent>();


    // Main thread for listening to new connections
    thread::spawn(move || {
        // This is put into a separate thread because it blocks on the listener, and we don't want that to block 
        // listening to the currently connected clients. I don't want to make the listener non-blocking because I don't 
        // want to write error handling for that.

        let mut connection_counter: u64 = 0;
        let server_host = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
        let server_port = env::var("SERVER_PORT").unwrap_or(String::from("23"));

        let tcp_listener = TcpListener::bind(format!("{}:{}", server_host, server_port))
            .expect("Error starting TCP listener");

        for connection_result in tcp_listener.incoming() {
            let mut connection = match connection_result {
                Ok(conn) => conn,
                Err(err) => {
                    println!("Error accepting connection: {}", err);
                    break;
                }
            };

            if let Err(err) = connection.set_nonblocking(true) {
                println!("Failed to set to non-blocking: {}", err);
                break;
            }

            if let Err(err) = connection.write_all(GREETING.as_bytes()) {
                println!("Failed to write greeting, closing connection: {}", err);
                break;
            };

            if let Err(err) = between_threads_tx.send(NetworkConnection { id: connection_counter, conn: connection }) {
                println!("Failed to send connection to managing thread: {}", err);
                break;
            };

            connection_counter += 1;
        }
    });

    // Sends new connections to the game world, along with new commands or GMCP commands. Also disconnects.
    thread::spawn(move || {
        let mut all_connections = Vec::<NetworkConnection>::new();
        loop {
            let mut to_remove = Vec::<u64>::new();

            // Add new connections to our list, then pass them on to the junction that will send it to the game world
            let new_conn = match between_threads_rx.try_recv() {
                Ok(conn) => conn,
                Err(err) => {
                    if err == TryRecvError::Empty {
                        continue;
                    }

                    println!("Error communicating between threads: {}", err);
                    break;
                },
            };

            if let Err(err) = connection_event_tx.send(NetworkEvent {
                id: new_conn.id,
                data: None,
                event_type: NetworkEventType::NewConnection,
            }) {
                println!("Failed to send network event to junction: {}", err);
                break;
            };

            all_connections.push(new_conn);

            // Process outgoing events
            loop {
                let outgoing_event = match outgoing_event_rx.try_recv() {
                    Ok(event) => event,
                    Err(err) => {
                        if err == TryRecvError::Empty {
                            continue;
                        }

                        println!("Error receiving from outgoing event: {}", err);
                        break;
                    },
                };

                let outgoing_event_connection = match all_connections.iter_mut().find(|conn| conn.id == outgoing_event.id) {
                    Some(conn) => conn,
                    None => continue,
                };

                let outgoing_text = match outgoing_event.text {
                    Some(text) => text,
                    None => continue,
                };

                if outgoing_event_connection.conn.write_all(&outgoing_text).is_ok() {
                    continue;
                }

                // Connection closed
                to_remove.push(outgoing_event_connection.id);

                if let Err(_) = outgoing_event_connection.conn.shutdown(std::net::Shutdown::Both){
                    println!("Failed to shutdown connection");
                    continue;
                }

                // Send the connection dropped event to the game because we can't write to them anymore
                if let Err(err) = connection_event_tx.send(NetworkEvent {
                    id: outgoing_event_connection.id,
                    data: None,
                    event_type: NetworkEventType::ConnectionDropped,
                }){
                    let mut error_message: String = String::from("Failed to send connection dropped event: ");
                    error_message.push_str(&err.to_string());
                    println!("{}", error_message);
                };

                continue;
            }

            for network_connection in &mut all_connections {
                let mut buf = [0; 1024];

                match network_connection.conn.peek(&mut buf) {
                    Ok(0) => {
                        // Connection closed
                        if let Err(_) = network_connection.conn.shutdown(std::net::Shutdown::Both){
                            to_remove.push(network_connection.id);
                            println!("Failed to shutdown connection, still discarding.");
                            continue;
                        }

                        to_remove.push(network_connection.id);

                        let send_success = connection_event_tx.send(NetworkEvent {
                            data: None,
                            id: network_connection.id,
                            event_type: NetworkEventType::ConnectionDropped,
                        });

                        if let Err(err) = send_success {
                            println!("Failed to send connection dropped event: {}", err);
                        };
                        
                        continue;
                    }
                    Ok(_) => {
                        let mut reader = BufReader::new(&network_connection.conn);
                        let mut line = String::new();

                        if let Err(err) = reader.read_line(&mut line) {
                            println!("Error reading line: {}", err);
                            continue;
                        }

                        if let Err(err) = connection_event_tx.send(NetworkEvent {
                            data: Some(line.into_bytes()),
                            id: network_connection.id,
                            event_type: NetworkEventType::InputReceived,
                        }) {
                            println!("Failed to send network event to junction: {}", err);
                            continue;
                        }
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        // No data available yet
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
            println!("Failed to send outgoing event: {}", err);
        }
    }
}

/// Handles transferring new connections into the game world, and sending data from the game world to the client
fn transfer_from_server_to_game(
    connection_event_rx: NonSend<NewConnectionListener>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
    mut ev_new_connection: EventWriter<NewConnectionEvent>,
) {
    loop {
        let new_event = match connection_event_rx.0.try_recv() {
            Ok(event) => event,
            Err(_) => break,
        };

        match new_event.event_type {
            NetworkEventType::NewConnection => {
                outgoing_queue.0.push(OutgoingEvent {
                    id: new_event.id,
                    text: Some(
                        "Welcome to RinoraMUD! Please select an option: \n"
                            .as_bytes()
                            .to_vec(),
                    ),
                    gmcp: None,
                });

                ev_new_connection.send(NewConnectionEvent(new_event.id));
            }
            NetworkEventType::InputReceived => todo!("Input received"),
            NetworkEventType::ConnectionDropped => todo!("Connection dropped"),
            NetworkEventType::GmcpReceived => todo!("GMCP not implemented yet"),
        }
    }
}

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewConnectionEvent>()
            .add_startup_system(start_listening)
            .add_system(process_outgoing_data)
            .add_system(transfer_from_server_to_game);
    }
}
