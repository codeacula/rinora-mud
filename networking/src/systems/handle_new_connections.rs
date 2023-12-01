use shared::prelude::*;

use crate::{events::UserConnectedEvent, ConnectionToEntityMap};

pub(crate) fn handle_new_connections(
    mut user_connected_rx: EventReader<UserConnectedEvent>,
    mut connection_to_user_entity: ResMut<ConnectionToEntityMap>,
    mut commands: Commands,
) {
    for UserConnectedEvent(id) in user_connected_rx.read() {
        let user_sesh = UserSessionData {
            connection: *id,
            entity_they_are_controlling: None,
        };

        let needs_username = NeedsUsername {};

        let entity = commands.spawn((user_sesh, needs_username));

        connection_to_user_entity.0.insert(*id, entity.id());
    }
}
