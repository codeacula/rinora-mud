use std::env;

use tokio::net::TcpListener;

pub struct GameServer {
    listener: Option<TcpListener>,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer { listener: None }
    }

    pub async fn start_server(&mut self) {
        let server_host = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
        let server_port = env::var("SERVER_PORT").unwrap_or(String::from("23"));

        let listener = TcpListener::bind(format!("{}:{}", server_host, server_port))
            .await
            .unwrap();
        self.listener = Some(listener);
    }

    pub async fn start_game_loop(&mut self) {
        loop {
            self.listener.as_ref().unwrap().poll_accept(_a);
        }
    }
}
