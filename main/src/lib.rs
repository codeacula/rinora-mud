use std::{env, net::TcpListener, sync::mpsc, thread};

pub struct GameServer {
    listener: Option<TcpListener>,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer { listener: None }
    }

    pub async fn start_server(&mut self) {
        let (tx, rx) = mpsc::channel();
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
                        tx.send(conn).unwrap();
                    }
                    Err(e) => {
                        panic!("Error accepting connection: {}", e);
                    }
                }
            }
        });
    }

    pub async fn start_game_loop(&mut self) {
        loop {}
    }
}
