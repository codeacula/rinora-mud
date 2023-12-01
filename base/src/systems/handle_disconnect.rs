use shared::prelude::*;

use crate::events::*;

/// When a user disconnects
pub fn handle_disconnect(
    mut ev_disconnection_event: EventReader<DisconnectionEvent>,
    mut ev_entity_left_room: EventWriter<EntityLeftRoomEvent>,
    mut ev_entity_left_world: EventWriter<EntityLeftWorldEvent>,
    query: Query<&UserSessionData>,
    character_info_query: Query<&Location>,
    room_map: ResMut<RoomMap>,
    mut commands: Commands,
) {
    for ev in ev_disconnection_event.read() {
        let Ok(user) = query.get(ev.entity) else {
            error!("User disconnected but no user component found");
            continue;
        };

        // If they were controlling an entity (probably a character), we need to make sure we either remove the
        // IsControlledBy tag, or we need to remove the entity from the world and store it.
        if user.entity_they_are_controlling.is_some() {
            let controlled_entity = user.entity_they_are_controlling.unwrap();
            let found_location = character_info_query.get(controlled_entity);

            let Ok(location) = found_location else {
                continue;
            };

            let Some(room) = room_map.0.get(&location.location_id) else {
                continue;
            };

            ev_entity_left_room.send(EntityLeftRoomEvent {
                entity: controlled_entity,
                room_entity_was_in: *room,
                message: String::from("Someone suddenly vanished."),
            });

            ev_entity_left_world.send(EntityLeftWorldEvent {
                entity: controlled_entity,
                room_entity_was_in: *room,
            });
        }

        commands.entity(ev.entity).despawn_recursive();
    }
}
