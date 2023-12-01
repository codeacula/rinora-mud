use crate::{events::UserDisconnectedEvent, ConnectionToEntityMap};
use shared::prelude::*;

pub(crate) fn handle_user_disconnected(
    mut user_disconnected_rx: EventReader<UserDisconnectedEvent>,
    conn_to_entity_map: Res<ConnectionToEntityMap>,
    mut commands: Commands,
) {
    for UserDisconnectedEvent(id) in user_disconnected_rx.read() {
        let entity = match conn_to_entity_map.0.get(id) {
            Some(entity) => *entity,
            None => {
                error!("User disconnected but we don't have a record of them");
                continue;
            }
        };

        commands.entity(entity).insert(LogOutUser {});
    }
}
