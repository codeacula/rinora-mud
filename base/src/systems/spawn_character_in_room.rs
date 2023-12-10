use shared::prelude::*;

pub(crate) fn spawn_character_in_room(
    mut character_logged_in_rx: EventReader<CharacterLoggedInEvent>,
    mut entity_collection_query: Query<&mut EntityCollection, With<Room>>,
) {
    for CharacterLoggedInEvent(character_entity) in character_logged_in_rx.read() {
        let entity = *character_entity;

        let mut entity_collection = match entity_collection_query.get_mut(entity) {
            Ok(data) => data,
            Err(_) => continue,
        };

        entity_collection.0.push(entity);
    }
}
