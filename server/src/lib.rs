pub struct ServerPlugin;

use std::net::{TcpListener, TcpStream};

use bevy::{ecs::system::Commands, prelude::*};

#[derive(Resource)]
struct Server(TcpListener);

struct ConnectionEvent {
  stream: TcpStream,
};

fn start_listening(mut commands: Commands) {
  let listener = match TcpListener::bind("0.0.0.0:23") {
    Ok(listener) => listener,
    Err(e) => panic!("{:?}", e),
  };

  let server = Server(listener);
  commands.insert_resource(server);
}

fn check_connections(server: ResMut<Server>, mut event_writer: EventWriter<ConnectionEvent>) {
  for conn in server.0.incoming() {
    match conn {
      Ok(newStream) => event_writer.send(ConnectionEvent { stream: newStream }),
      Err(err) => println!("Um, err? {}", err),
    }
  }
}

impl Plugin for ServerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(start_listening)
      .add_system(check_connections);
  }
}
