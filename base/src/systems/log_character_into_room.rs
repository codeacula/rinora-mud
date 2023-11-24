use shared::prelude::*;

pub fn log_character_into_room(
    characters_logging_in: Query<
        (Entity, &Location),
        (With<EntityIsLoggingIn>, With<IsControlledBy>),
    >,
    mut room_query: Query<&mut EntityCollection, With<Room>>,
    mut entity_entered_room_tx: EventWriter<EntityEnteredRoomEvent>,
) {
    for (entity, location) in characters_logging_in.iter() {
        let mut entities = room_query.get_mut(location.entity).unwrap();
        entities.0.push(entity);

        entity_entered_room_tx.send(EntityEnteredRoomEvent {
            entity,
            room_entity_is_in: location.entity,
        });
    }
}
