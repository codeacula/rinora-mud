use shared::prelude::*;

pub fn display_character_entering_room(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoomEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    characters_in_world: Query<&DisplayName, With<Character>>,
    room_info_query: Query<&EntityCollection, With<Room>>,
) {
    for ev in entity_entered_room_rx.read() {
        let Ok(display_name) = characters_in_world.get(ev.entity) else {
            info!("Entity entering has no display name.");
            continue;
        };

        let Ok(collection) = room_info_query.get(ev.room_entity_is_in) else {
            info!("Room being entered into has no EntityCollection");
            continue;
        };

        if collection.0.is_empty() {
            info!("Room doesn't have any attached entities in its collection.");
            continue;
        }

        for entity_in_room in collection.0.iter() {
            let Ok(controlling_entity) = is_controlled_by_query.get(*entity_in_room) else {
                continue;
            };

            if ev.entity.eq(entity_in_room) {
                text_event_tx.send(TextEvent::new(
                    controlling_entity.0,
                    &format!("You wander off {}.", "somewhere"),
                ));
            } else {
                text_event_tx.send(TextEvent::new(
                    controlling_entity.0,
                    &format!("{} has entered the location.", display_name.0.clone()),
                ));
            }
        }
    }
}
