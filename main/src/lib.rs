mod game_server;

use bevy_ecs::prelude::*;
use game_server::GameServer;

pub fn start_game() {
    let mut world = World::new();

    let server = GameServer::new();

    let mut schedule = Schedule::default();

    let connection_activity_listener = server.connection_event_listener;

    println!("Starting game loop");
    loop {
        let connection_event = match connection_activity_listener.try_recv() {
            Err(_) => None,
            Ok(conn) => Some(conn),
        };

        if connection_event.is_some() {
            let event_info = connection_event.unwrap();

            match event_info.event_type {
                game_server::ConnectionEventTypes::ConnectionDropped {} => {
                    println!("Connection dropped");
                },
                game_server::ConnectionEventTypes::DataReceived {} => {
                    println!("Got data");
                },
                game_server::ConnectionEventTypes::NewConnection {} => {
                    println!("New connection");
                },
            }
        }

        schedule.run(&mut world);
    }
}
