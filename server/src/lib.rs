use std::{
    env,
    io::{BufRead, BufReader, ErrorKind, Write},
    net::TcpListener,
    net::TcpStream,
    sync::mpsc::*,
    thread,
};

use bevy::prelude::*;

pub struct GameServer;

/// What type of events the server will issue the game
pub enum NetworkEventType {
    NewConnection,
    InputReceived,
    ConnectionDropped,
    GmcpReceived,
}

/// A network
pub struct NetworkEvent {
    pub id: u64,
    pub data: Option<Vec<u8>>,
    pub event_type: NetworkEventType,
}

#[derive(Debug)]
pub struct GameConnection {
    id: u64,
    conn: TcpStream,
}

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

pub struct NetworkListener(Receiver<NetworkEvent>);

fn start_listening(world: &mut World) {
    let (connection_event_tx, connection_event_rx) = channel();
    let (between_threads_tx, between_threads_rx) = channel();

    thread::spawn(move || {
        let mut counter: u64 = 0;
        let server_host = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
        let server_port = env::var("SERVER_PORT").unwrap_or(String::from("23"));

        let listener = TcpListener::bind(format!("{}:{}", server_host, server_port))
            .expect("Error starting TCP listener");

        for conn in listener.incoming() {
            if let Ok(mut conn) = conn {
                conn.write_all(b"Beware, friends, for peril and challenge lurk inside...\n")
                    .expect("Failed to send message");
                conn.write_all(b"Built on the RinoraMUD engine alpha")
                    .expect("Failed to send message");

                conn.set_nonblocking(true)
                    .expect("Failed to set to non-blocking");
                between_threads_tx
                    .send(GameConnection { id: counter, conn })
                    .expect("Failed to send connection between threads");
                counter += 1;
            }
        }
    });

    thread::spawn(move || -> ! {
        let mut connections = Vec::<GameConnection>::new();
        loop {
            if let Ok(new_conn) = between_threads_rx.try_recv() {
                connection_event_tx
                    .send(NetworkEvent {
                        id: new_conn.id,
                        data: None,
                        event_type: NetworkEventType::NewConnection,
                    })
                    .expect("Failed to send new connection event");
                connections.push(new_conn);
            }

            let mut to_remove = Vec::<u64>::new();

            for game_connection in &mut connections {
                let mut buf = [0; 1024];

                match game_connection.conn.peek(&mut buf) {
                    Ok(0) => {
                        // Connection closed
                        game_connection
                            .conn
                            .shutdown(std::net::Shutdown::Both)
                            .unwrap_or_default();
                        to_remove.push(game_connection.id);

                        connection_event_tx
                            .send(NetworkEvent {
                                data: None,
                                id: game_connection.id,
                                event_type: NetworkEventType::ConnectionDropped,
                            })
                            .unwrap();
                        continue;
                    }
                    Ok(_) => {
                        let mut reader = BufReader::new(&game_connection.conn);
                        let mut line = String::new();

                        if reader.read_line(&mut line).is_ok() {
                            game_connection
                                .conn
                                .write_all("You said: ".as_bytes())
                                .unwrap();
                            game_connection.conn.write_all(line.as_bytes()).unwrap();
                            game_connection.conn.write_all("\n".as_bytes()).unwrap();
                            connection_event_tx
                                .send(NetworkEvent {
                                    data: Some(line.into_bytes()),
                                    id: game_connection.id,
                                    event_type: NetworkEventType::InputReceived,
                                })
                                .unwrap();
                        }
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        // No data available yet
                    }
                    Err(e) => panic!("Unexpected error: {}", e),
                }
            }

            for id in to_remove {
                connections.retain(|conn| conn.id != id);
            }
        }
    });

    world.insert_non_send_resource(NetworkListener(connection_event_rx));
}

impl Plugin for GameServer {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_listening);
    }
}
