use std::{
    env,
    io::{Write, BufReader, BufRead, ErrorKind},
    net::TcpListener,
    net::TcpStream,
    sync::mpsc::*,
    thread,
};

pub enum ConnectionEventTypes {
    NewConnection,
    DataReceived,
    ConnectionDropped,
}

pub struct ConnectionEvent {
    pub id: u64,
    pub data: Option<Vec<u8>>,
    pub event_type: ConnectionEventTypes,
}

#[derive(Debug)]
pub struct GameConnection {
    id: u64,
    conn: TcpStream,
}

#[derive(Debug)]
pub struct GameServer {
    pub connection_event_listener: Receiver<ConnectionEvent>,
}

fn start_server_thread() -> Receiver<ConnectionEvent> {
    let (connection_event_tx, connection_event_rx) = channel();
    let (between_threads_tx, between_threads_rx) = channel();

    thread::spawn(move || {
        let mut counter: u64 = 0;
        let server_host = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
        let server_port = env::var("SERVER_PORT").unwrap_or(String::from("23"));

        let listener = TcpListener::bind(format!("{}:{}", server_host, server_port)).expect("Error starting TCP listener");

        for conn in listener.incoming() {
            if let Ok(mut conn) = conn {
                conn.write_all(b"Beware, friends, for peril and challenge lurk inside...\n").expect("Failed to send message");
                conn.write_all(b"Built on the RinoraMUD engine alpha").expect("Failed to send message");

                conn.set_nonblocking(true).expect("Failed to set to non-blocking");
                between_threads_tx.send(GameConnection {
                    id: counter,
                    conn
                }).expect("Failed to send connection between threads");
                counter += 1;
            }
        }
    });

    thread::spawn(move || -> ! {
        let mut connections = Vec::<GameConnection>::new();
        loop {
            if let Ok(new_conn) = between_threads_rx.try_recv() {
                connection_event_tx.send(ConnectionEvent{
                    id: new_conn.id,
                    data: None,
                    event_type: ConnectionEventTypes::NewConnection
                }).expect("Failed to send new connection event");
                connections.push(new_conn);
            }

            let mut to_remove = Vec::<u64>::new();

            for game_connection in &mut connections {
                let mut buf = [0; 1024];

                match game_connection.conn.peek(&mut buf) {
                    Ok(0) => {
                        // Connection closed
                        game_connection.conn.shutdown(std::net::Shutdown::Both).unwrap_or_default();
                        to_remove.push(game_connection.id);

                        connection_event_tx.send(ConnectionEvent{
                            data: None,
                            id: game_connection.id,
                            event_type: ConnectionEventTypes::ConnectionDropped
                        }).unwrap();
                        continue;
                    },
                    Ok(_) => {
                        let mut reader = BufReader::new(&game_connection.conn);
                        let mut line = String::new();

                        if reader.read_line(&mut line).is_ok() {
                        
                            game_connection.conn.write_all("You said: ".as_bytes()).unwrap();
                            game_connection.conn.write_all(line.as_bytes()).unwrap();
                            game_connection.conn.write_all("\n".as_bytes()).unwrap();
                            connection_event_tx.send(ConnectionEvent{
                                data: Some(line.into_bytes()),
                                id: game_connection.id,
                                event_type: ConnectionEventTypes::DataReceived
                            }).unwrap();
                        }
                    },
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        // No data available yet
                    },
                    Err(e) => panic!("Unexpected error: {}", e),
                }
            }

            for id in to_remove {
                connections.retain(|conn| conn.id != id);
            }
        }
    });

    return connection_event_rx;
}

impl GameServer {
    pub fn new() -> GameServer {
        println!("Starting server!");
        let connection_event_rx = start_server_thread();

        GameServer {
            connection_event_listener: connection_event_rx,
        }
    }
}
