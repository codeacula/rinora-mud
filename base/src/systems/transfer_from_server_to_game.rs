use shared::prelude::*;

use crate::{enums::*, resources::*};

/// Handles transferring new connections into the game world, and sending data from the game world to the client
pub fn transfer_from_server_to_game(
    connection_event_rx: NonSend<NewConnectionListener>,
    mut ev_new_connection: EventWriter<NewConnectionEvent>,
    mut ev_dropped_connection: EventWriter<DisconnectionEvent>,
    mut ev_input_received_connection: EventWriter<InputReceivedEvent>,
    mut ev_gmcp_received_connection: EventWriter<GmcpReceivedEvent>,
    mut network_info: ResMut<NetworkInfo>,
    mut commands: Commands,
) {
    while let Ok(new_event) = connection_event_rx.0.try_recv() {
        match new_event.event_type {
            NetworkEventType::NewConnection => {
                let entity = commands
                    .spawn((UserSessionData {
                        connection: new_event.id,
                        pwd: None,
                        status: UserStatus::NeedUsername,
                        username: String::new(),
                        char_to_delete: None,
                        controlling_entity: None,
                    },))
                    .id();
                network_info
                    .connection_to_entity
                    .insert(new_event.id, entity);
                ev_new_connection.send(NewConnectionEvent {
                    entity,
                    id: new_event.id,
                });
                debug!("Spawned user session: {:?}", entity);
            }
            NetworkEventType::InputReceived => {
                let entity = *network_info
                    .connection_to_entity
                    .get(&new_event.id)
                    .unwrap();

                let input = String::from_utf8(new_event.data.unwrap()).unwrap();

                // We don't want to bother processing empty requests
                if input.trim().is_empty() {
                    continue;
                }

                ev_input_received_connection.send(InputReceivedEvent { entity, input });
            }
            NetworkEventType::ConnectionDropped => {
                let entity = network_info
                    .connection_to_entity
                    .remove(&new_event.id)
                    .unwrap();
                ev_dropped_connection.send(DisconnectionEvent { entity });
            }
            NetworkEventType::GmcpReceived => {
                let entity = *network_info
                    .connection_to_entity
                    .get(&new_event.id)
                    .unwrap();
                ev_gmcp_received_connection.send(GmcpReceivedEvent {
                    entity,
                    data: new_event.data.unwrap(),
                });
            }
        }
    }
}
