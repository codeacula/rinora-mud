pub struct ServerPlugin {
    pub host: String,
    pub port: String,
}
use std::io::{Read, Write};
use std::{
    net::{TcpListener, TcpStream},
};

use bevy::{ecs::system::Commands, prelude::*};

#[derive(Component)]
struct Connection {
    stream: TcpStream,
}

enum NetworkEventType {
    Connected,
    Disconnected,
    DataReceived,
    DataSent,
}

struct NetworkEvent {
    event_type: NetworkEventType,
    stream: TcpStream,
}

#[derive(Resource)]
struct Server {
    listener: TcpListener,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn check_for_new_connections(mut commands: Commands, server: Res<Server>) {
    for new_connection in server.listener.incoming() {
        let mut stream = match new_connection {
            Ok(conn) => conn,
            Err(_e) => {
                break;
            }
        };

        if let Err(_) = stream.write(String::from("Welcome to RinoraMUD!").as_bytes()) {
            eprintln!("Failed to respond to client");
        }
        let new_entity = commands.spawn(Connection { stream });

    }
}

fn check_for_waiting_input(all_connections: Query<&Connection>) {
    for conn in all_connections.iter() {
        let mut buf = [0; 1024];
        let mut stream = conn.stream.try_clone().unwrap();

        match stream.read(&mut buf) {
            Ok(size) => {
                if size > 0 {
                    let mut message = String::from_utf8_lossy(&buf[..size]).into_owned();

                    // Process your GMCP data here.

                    // Respond back to the client.
                    if let Err(_) = stream.write(message.as_bytes()) {
                        eprintln!("Failed to respond to client");
                        break;
                    }
                } else if size == 0 {
                    // Connection was closed.
                    break;
                } else {
                    // An error occurred.
                    eprintln!("Failed to read from client");
                    break;
                }
            }
            Err(err) => {
                eprintln!("Failed to read from client: {:?}", err);
                break;
            }
        }
    }
}

fn insert_people(mut commands: Commands) {
    commands.spawn((Person, Name { 0: "Alice".to_string() }));
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

        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");

        let server = Server { listener };

        app.insert_resource(server)
            .add_event::<NetworkEvent>()
            .add_startup_system(insert_people)
            .add_system(check_for_new_connections)
            .add_system(check_for_waiting_input)
            .add_system(greet_people);
    }
}
