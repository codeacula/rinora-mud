use bevy::prelude::*;
use shared::{networking::*, user::User};

pub struct AccountPlugin;

/// Handles transferring new connections into the game world, and sending data from the game world to the client
fn handle_new_connections(
    mut commands: Commands,
    connection_event_rx: NonSend<NewConnectionListener>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    loop {
        let new_event = match connection_event_rx.0.try_recv() {
            Ok(event) => event,
            Err(_) => break,
        };

        match new_event.event_type {
            NetworkEventType::NewConnection => {
                outgoing_queue.0.push(OutgoingEvent {
                    id: new_event.id,
                    text: Some("Welcome to Rinora!\n".as_bytes().to_vec()),
                    gmcp: None,
                });

                commands.spawn(User {
                    connection: new_event.id,
                });
            }
            NetworkEventType::InputReceived => {
                println!("Input received: {}", new_event.id);
            }
            NetworkEventType::ConnectionDropped => {
                println!("Connection dropped: {}", new_event.id);
            }
            NetworkEventType::GmcpReceived => {
                println!("GMCP received: {}", new_event.id);
            }
        }
    }
}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_new_connections);
    }
}
