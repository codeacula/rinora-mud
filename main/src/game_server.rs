use std::{env, net::TcpListener, net::TcpStream, sync::mpsc::*, thread, io::{Write, Read}};

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
                    conn.write("Beware, friends, for peril and challenge lurk inside...".as_bytes()).unwrap();
                    conn.write("Built on the RinoraMUD engine alpha".as_bytes()).unwrap();

                    
                    between_threads_tx.send(conn.try_clone().unwrap()).unwrap();
                    new_conn_tx.send(conn.try_clone().unwrap()).unwrap();
                }
                Err(e) => {
                    panic!("Error accepting connection: {}", e);
                }
            }
        }
    });

    thread::spawn(move || {
        let mut connections = Vec::<TcpStream>::new();

        loop {
            let new_connection = between_threads_rx.recv().unwrap();
            connections.push(new_connection.try_clone().unwrap());

            for mut conn in &connections {
                let mut buf = [0; 1024];
                let bytes_read = conn.read(&mut buf).unwrap();
                if bytes_read > 0 {
                    println!("Received {} bytes from {:?}", bytes_read, conn);
                    conn_activity_tx.send(conn.try_clone().unwrap()).unwrap();
                }
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