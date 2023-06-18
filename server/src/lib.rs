pub struct ServerPlugin {
    pub host: String,
    pub port: String,
}

use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use bevy::{ecs::system::Commands, prelude::*};

#[derive(Component)]
struct Connection {
    stream: TcpStream,
}

#[derive(Resource)]
struct Server {
    listener: TcpListener
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


fn check_for_new_connections(mut commands: Commands, server: Res<Server>) {
    for new_connection in server.listener.incoming() {
        let conn = match new_connection {
            Ok(conn) => conn,
            Err(_e) => { continue; }
        };
        commands.spawn(Connection { stream: conn });
    }
}

fn check_for_waiting_input(all_connections: Query<&Connection>) {
    for conn in all_connections.iter() {
        let mut buf = String::new();
        let mut stream_copy = match conn.stream.try_clone() {
            Ok(stream) => stream,
            Err(e) => panic!("{:?}", e),
        };

        match stream_copy.read_to_string(&mut buf) {
            Err(err) => {
                panic!("{:?}", err);
            }
            _ => (),
        }

        if buf.len() > 0 {
            println!("Received: {}", buf);
        }
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {

        let listener = match TcpListener::bind(format!("{}:{}", self.host, self.port)) {
            Ok(listener) => listener,
            Err(e) => panic!("{:?}", e),
        };

        listener.set_nonblocking(true).expect("Cannot set non-blocking");

        let server = Server {
            listener,
        };

        app
            .insert_resource(server)
            .add_system(check_for_new_connections)
            .add_system(check_for_waiting_input)
            .add_system(greet_people);
    }
}
