use shared::prelude::*;

pub fn log_character_into_room(
    characters_logging_in: Query<(Entity, &Location), With<EntityIsLoggingIn>>,
    mut room_query: Query<(&Room, &mut EntityCollection)>,
) {
    for (entity, location) in characters_logging_in.iter() {
        let (room, mut entities) = room_query.get_mut(location.entity).unwrap();
        entities.0.push(entity);

        info!("Character logged in: {:#?} to room {room:?}", entity);
    }
}
