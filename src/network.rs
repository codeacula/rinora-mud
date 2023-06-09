use bevy::prelude::*;
use std::{env, net::TcpListener};
pub struct NetworkManagerPlugin {
  listener: TcpListener,
}

impl Plugin for NetworkManagerPlugin {
  fn build(&self, app: &mut App) {
    let host = match env::var("HOST") {
      Ok(val) => val,
      Err(_) => String::from("0.0.0.0"),
    };

    let port = 23;

    let listener = match TcpListener::bind(format!("{}:{}", host, port)) {
      Ok(val) => val,
      Err(e) => {
        panic!(format!("Unable to create a TcpListener on {}:{}", host, port))
    }
    }
  }
}
