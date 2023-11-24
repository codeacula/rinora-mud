use shared::prelude::*;

pub fn process_entities_that_want_to_move(
    mut entities_wanting_to_move: Query<(Entity, &EntityWantsToMove, &mut Location)>,
    exit_query: Query<&Exit>,
    mut room_query: Query<(&Room, &mut EntityCollection)>,
    mut entity_moved_rooms_tx: EventWriter<EntityMovedRooms>,
    mut entity_entered_room_tx: EventWriter<EntityEnteredRoomEvent>,
    mut commands: Commands,
) {
    for (moving_entity, wants_to_move, mut current_location) in entities_wanting_to_move.iter_mut()
    {
        let exit = exit_query
            .get(wants_to_move.exit_entity)
            .expect("No exit found");

        let (exit_to_room, mut entering_room_entity_collection) =
            room_query.get_mut(exit.to_room).expect("No room found");

        // Update the entity's location with the new room
        current_location.location_id = exit_to_room.room_id;
        current_location.entity = exit.to_room;
        commands.entity(moving_entity).remove::<EntityWantsToMove>();

        // Let the world know the entity moved rooms
        entity_moved_rooms_tx.send(EntityMovedRooms {
            moving_entity,
            from_room: exit.from_room,
            to_room: exit.to_room,
        });

        // Add the entity to the new room's entity collection, and remove it from the old one
        entering_room_entity_collection.0.push(moving_entity);

        entity_entered_room_tx.send(EntityEnteredRoomEvent {
            entity: moving_entity,
            room_entity_is_in: exit.to_room,
        });

        let (_exit_from_room, mut exiting_room_entity_collection) =
            room_query.get_mut(exit.from_room).expect("No room found");

        exiting_room_entity_collection
            .0
            .retain(|e| e != &moving_entity);
    }
}
