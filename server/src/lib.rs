pub struct ServerPlugin;

use std::net::TcpListener;

use bevy::{ecs::system::Commands, prelude::*};

#[derive(Resource)]
struct Server(TcpListener);

fn start_listening(mut commands: Commands) {
  let listener = match TcpListener::bind("0.0.0.0:23") {
    Ok(l) => l,
    Err(e) => panic!("{:?}", e),
  };

  let server = Server(listener);
  commands.insert_resource(server);
}

fn check_connections(server: ResMut<Server>) {
  for conn in server.0.incoming() {
    match conn {
      Ok(_stream) => println!("We have a connection!"),
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
