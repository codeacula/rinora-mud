use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
    net::TcpStream,
    sync::mpsc::*,
    thread,
};

struct GameConnection {
    id: u64,
    conn: TcpStream,
}

pub struct GameServer {
    pub connection_activity_listener: Receiver<TcpStream>,
    pub new_connection_listener: Receiver<TcpStream>,
    pub connections: Vec<TcpStream>,
}

fn start_server_thread() -> (Receiver<TcpStream>, Receiver<TcpStream>) {
    let (new_conn_tx, new_conn_rx) = channel();
    let (conn_activity_tx, conn_activity_rx) = channel();
    let (between_threads_tx, between_threads_rx) = channel();

    thread::spawn(move || {
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
                    println!("Passing new connection: {:?}", conn);
                    conn.write(
                        "Beware, friends, for peril and challenge lurk inside...\n".as_bytes(),
                    )
                    .unwrap();
                    conn.write("     Built on the RinoraMUD engine alpha".as_bytes())
                        .unwrap();

                    between_threads_tx.send(conn.try_clone().unwrap()).unwrap();
                    new_conn_tx.send(conn.try_clone().unwrap()).unwrap();
                }
                Err(e) => {
                    panic!("Error accepting connection: {}", e);
                }
            }
        }
    });

    thread::spawn(move || -> ! {
        let mut connections = Vec::<GameConnection>::new();
        let mut counter: u64 = 0;
        loop {
            let new_conn = match between_threads_rx.try_recv() {
                Err(_) => None,
                Ok(conn) => Some(conn),
            };

            if new_conn.is_some() {
                connections.push(GameConnection {
                    id: counter,
                    conn: new_conn.unwrap(),
                });
                counter += 1;
            }

            let mut to_remove = Vec::<u64>::new();

            for game_connection in &mut connections {
                let mut buf = Vec::<u8>::new();
                let mut conn = game_connection.conn.try_clone().unwrap();
                let bytes_read = conn.read_to_end(&mut buf).unwrap();
                if bytes_read > 0 {
                    println!("Received {} bytes from {:?}", bytes_read, conn);
                    conn_activity_tx.send(conn).unwrap();
                } else if bytes_read == 0 {
                    println!("Connection closed: {:?}", conn);
                    game_connection
                        .conn
                        .shutdown(std::net::Shutdown::Both)
                        .unwrap();
                    to_remove.push(game_connection.id);
                }
                println!("Restarting loop");
            }

            for id in to_remove {
                connections.retain(|conn| conn.id != id);
            }
        }
    });

    return (new_conn_rx, conn_activity_rx);
}

impl GameServer {
    pub fn new() -> GameServer {
        println!("Starting server!");
        let (new_conn_rx, conn_activity_rx) = start_server_thread();

        GameServer {
            connections: Vec::<TcpStream>::new(),
            connection_activity_listener: conn_activity_rx,
            new_connection_listener: new_conn_rx,
        }
    }
}
