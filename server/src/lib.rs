pub struct ServerPlugin;

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
  listener: TcpListener,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


fn start_listening(mut commands: Commands) {
  let listener = match TcpListener::bind("0.0.0.0:23") {
    Ok(listener) => listener,
    Err(e) => panic!("{:?}", e),
  };

  let server = Server { listener };
  commands.insert_resource(server);
}

fn check_for_new_connections(mut commands: Commands, server: Res<Server>) {
  for conn in server.listener.incoming() {
    match conn {
      Ok(stream) => {
        commands.spawn((Connection { stream },));
        commands.spawn((Person, Name("Elaina Proctor".to_string())));
        println!("New connection!");
      }
      Err(err) => println!("Um, err? {}", err),
    }
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
    app
      .add_startup_system(start_listening)
      .add_system(check_for_new_connections)
      .add_system(check_for_waiting_input)
      .add_system(greet_people)
      ;
  }
}
