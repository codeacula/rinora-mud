use bevy::prelude::*;
use shared::prelude::*;

pub fn display_character_logged_into_room(
    mut character_logged_in_event_rx: EventReader<CharacterLoggedInEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    characters_in_world: Query<(&DisplayName, &Location), With<Character>>,
    room_info_query: Query<&EntityCollection, With<Room>>,
    mut show_prompt_event_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in character_logged_in_event_rx.read() {
        let Ok((display_name, location)) = characters_in_world.get(ev.0) else {
            info!("Entity entering has no display name and location.");
            continue;
        };

        let Ok(collection) = room_info_query.get(location.entity) else {
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

            if ev.0.eq(entity_in_room) {
                text_event_tx.send(TextEvent::new(
                    controlling_entity.0,
                    &format!("You find yourself disoriented, a blinding bright light filling your vision as your soul leaves suspension. The light pulses as it burns, warm and comforting. After a moment you feel a lurch in your guts, your soul flung from the heart of Ero'ghal and back to the planes it calls home. Almost instantly, you open your eyes and find yourself safe in {} upon the {} plane.", "the Wild Plains", "mortal"),
                ));
            } else {
                text_event_tx.send(TextEvent::new(
                    controlling_entity.0,
                    &format!("Reality bends for a moment as {}'s soul exits suspension, their body appearing, radiating a glowing light that casts no shadows before the world settles back to normal.", display_name.0.clone()),
                ));
            }

            show_prompt_event_tx.send(ShowPromptEvent(controlling_entity.0));
        }
    }
}
