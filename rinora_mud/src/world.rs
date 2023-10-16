use bevy::prelude::*;
use shared::prelude::*;

pub fn add_character_to_room(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoom>,
    mut all_rooms: Query<&mut EntityCollection, With<Room>>,
) {
    for ev in entity_entered_room_rx.iter() {
        if let Ok(mut entity_collection) = all_rooms.get_mut(ev.room_entity_is_in) {
            entity_collection.0.push(ev.entity);
        }
    }
}
