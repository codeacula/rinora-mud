use std::{
    env,
    io::{Read, Write},
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

        let listener = match TcpListener::bind(format!("{}:{}", server_host, server_port)) {
            Ok(listener) => listener,
            Err(e) => {
                panic!("Error starting TCP listener: {}", e);
            }
        };

        for conn in listener.incoming() {
            match conn {
                Ok(mut conn) => {
                    conn.write("Beware, friends, for peril and challenge lurk inside...\n".as_bytes(),).unwrap();
                    conn.write("     Built on the RinoraMUD engine alpha".as_bytes()).unwrap();

                    between_threads_tx.send(GameConnection {
                        id: counter,
                        conn
                    }).unwrap();
                    counter += 1;
                }
                Err(e) => {
                    panic!("Error accepting connection: {}", e);
                }
            }
        }
    });

    thread::spawn(move || -> ! {
        let mut connections = Vec::<GameConnection>::new();
        loop {
            let new_conn = match between_threads_rx.try_recv() {
                Err(_) => None,
                Ok(conn) => Some(conn),
            };

            if new_conn.is_some() {
                connection_event_tx.send(ConnectionEvent{
                    id: new_conn.as_ref().unwrap().id,
                    data: None,
                    event_type: ConnectionEventTypes::NewConnection
                }).unwrap();
                connections.push(new_conn.unwrap());
            }

            let mut to_remove = Vec::<u64>::new();

            for game_connection in &mut connections {
                let mut buf = Vec::<u8>::new();
                let bytes_read = game_connection.conn.read_to_end(&mut buf).unwrap();
                if bytes_read > 0 {
                    println!("Received {} bytes from {:?}", bytes_read, game_connection.conn);
                    connection_event_tx.send(ConnectionEvent{
                        data: Some(buf),
                        id: game_connection.id,
                        event_type: ConnectionEventTypes::DataReceived
                    }).unwrap();
                } else if bytes_read == 0 {
                    println!("Connection closed: {:?}", game_connection.conn);

                    game_connection
                        .conn
                        .shutdown(std::net::Shutdown::Both)
                        .unwrap();

                    connection_event_tx.send(ConnectionEvent{
                        data: None,
                        id: game_connection.id,
                        event_type: ConnectionEventTypes::ConnectionDropped
                    }).unwrap();

                    to_remove.push(game_connection.id);
                }
                println!("Restarting loop");
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
