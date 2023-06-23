mod game_server;

use std::time::Duration;

use bevy_ecs::prelude::*;
use game_server::GameServer;

pub fn start_game() {
    let mut world = World::new();

    let server = GameServer::new();

    let mut schedule = Schedule::default();

    let new_connection_listener = server.new_connection_listener;

    println!("Starting game loop");
    loop {
        let new_connection = match new_connection_listener.recv_timeout(Duration::from_millis(0)) {
            Err(_) => None,
            Ok(conn) => Some(conn),
        };

        if new_connection.is_some() {
            println!("New connection: {:?}", new_connection);
        }

        schedule.run(&mut world);
    }
}
