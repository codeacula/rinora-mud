mod game_server;

use bevy_ecs::prelude::*;
use game_server::GameServer;

pub fn start_game() {
    let mut world = World::new();

    let server = GameServer::new();

    let mut schedule = Schedule::default();

    let new_connection_listener = server.new_connection_listener;

    println!("Starting game loop");
    loop {
        let new_connection = match new_connection_listener.try_recv() {
            Err(_) => None,
            Ok(conn) => Some(conn),
        };

        if new_connection.is_some() {
            println!("New connection: {:?}", new_connection);
        }

        schedule.run(&mut world);
    }
}
