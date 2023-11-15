use shared::prelude::*;

/// This method will:
/// - Update the entity's location to the new room.
/// - Fire an event to let the room know that the entity left
/// - Fire an event to let the room know an entity arrived
///
/// This runs in `Pre` because we want on-room events to fire in `Game`.
pub fn move_entity_to_room(
    mut move_entity_to_room_rx: EventReader<MoveEntityToRoom>,
    mut location_query: Query<&mut Location>,
    room_query: Query<&Room>,
    mut entity_left_room_ev: EventWriter<EntityLeftRoomEvent>,
    mut entity_entered_room_ev: EventWriter<EntityEnteredRoomEvent>,
) {
    for ev in move_entity_to_room_rx.read() {
        let mut entity_location = match location_query.get_mut(ev.entity) {
            Ok(location) => location,
            Err(_) => continue,
        };

        entity_left_room_ev.send(EntityLeftRoomEvent {
            entity: ev.entity,
            room_entity_was_in: entity_location.entity,
        });

        // Get the new room
        let new_room = match room_query.get(ev.room) {
            Ok(room) => room,
            Err(_) => {
                error!("Entity moved to a room that doesn't exist: {:?}", ev.room);
                continue;
            }
        };
        // Notify new room of arrival
        entity_entered_room_ev.send(EntityEnteredRoomEvent {
            entity: ev.entity,
            room_entity_is_in: ev.room,
        });

        // Update the entity's location
        entity_location.entity = ev.room;
        entity_location.location_id = new_room.room_id;
    }
}
