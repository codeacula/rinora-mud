use shared::prelude::*;

/// This method will:
/// - Update the entity's location to the new room.
/// - Fire an event to let the room know that the entity left
/// - Fire an event to let the room know an entity arrived
///
/// This runs in `Pre` because we want on-room events to fire in `Game`.
pub(crate) fn move_entity_to_room_via_event(
    mut move_entity_to_room_rx: EventReader<MoveEntityToRoom>,
    mut location_query: Query<&mut Location>,
    mut room_query: Query<(&Room, &mut EntityCollection)>,
    mut entity_left_room_ev: EventWriter<EntityLeftRoomEvent>,
    mut entity_entered_room_ev: EventWriter<EntityEnteredRoomEvent>,
) {
    for ev in move_entity_to_room_rx.read() {
        let mut entity_location = match location_query.get_mut(ev.entity) {
            Ok(location) => location,
            Err(_) => continue,
        };

        // Update the old room's entity collection
        if let Ok((_, mut old_room_collection)) = room_query.get_mut(entity_location.entity) {
            // Find and remove entity from old_room_collection
            if let Some(index) = old_room_collection
                .0
                .iter()
                .position(|&entity| entity == ev.entity)
            {
                old_room_collection.0.remove(index);
            }

            entity_left_room_ev.send(EntityLeftRoomEvent {
                entity: ev.entity,
                room_entity_was_in: entity_location.entity,
                message: String::from("Someone saunters away."),
            });
        };

        // Get the new room
        let (new_room, mut new_entity_collection) = match room_query.get_mut(ev.room) {
            Ok(collection) => collection,
            Err(e) => {
                error!(
                    "Entity moved to a room that doesn't exist: {:?} - {e:?}",
                    ev.room
                );
                continue;
            }
        };

        // Update the room's collection of entities
        new_entity_collection.0.push(ev.entity);

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
