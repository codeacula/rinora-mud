use std::{env, net::TcpListener, net::TcpStream, sync::mpsc::*, thread};

pub struct GameServer {
    pub connection_activity_listener: Option<Receiver<TcpStream>>,
    pub new_connection_listener: Receiver<TcpStream>,
}

impl GameServer {
    pub fn new() -> GameServer {
        println!("Starting server!");
        let (tx, rx) = channel();
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
                    Ok(conn) => {
                        println!("Passing new connection: {:?}", conn);
                        tx.send(conn).unwrap();
                    }
                    Err(e) => {
                        panic!("Error accepting connection: {}", e);
                    }
                }
            }
        });

        GameServer {
            connection_activity_listener: None,
            new_connection_listener: rx,
        }
    }
}
