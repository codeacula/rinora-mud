pub struct ServerPlugin;

use std::{
  io::{Read, Write},
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

struct ConnectionEvent {
  pub connection: Entity,
}

fn start_listening(mut commands: Commands) {
  let listener = match TcpListener::bind("0.0.0.0:23") {
    Ok(listener) => listener,
    Err(e) => panic!("{:?}", e),
  };

  let server = Server { listener };
  commands.insert_resource(server);
}

fn check_for_new_connections(
  mut commands: Commands,
  server: Res<Server>,
  mut event_writer: EventWriter<ConnectionEvent>,
) {
  for conn in server.listener.incoming() {
    match conn {
      Ok(stream) => {
        let connection = Connection { stream };
        let cmds = commands.spawn(connection);

        connection.stream.write(String::from("Ahoy!").as_bytes());

        event_writer.send(ConnectionEvent {
          connection: cmds.id(),
        });
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

    let _ = stream_copy.read_to_string(&mut buf);

    if buf.len() > 0 {
      println!("Received: {}", buf);
    }
  }
}

impl Plugin for ServerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(start_listening)
      .add_event::<ConnectionEvent>()
      .add_system(check_for_new_connections)
      .add_system(check_for_waiting_input);
  }
}
