use bevy::prelude::*;
use shared::prelude::*;

fn send_room_description(
    target: Entity,
    name: String,
    description: String,
    text_event_tx: &mut EventWriter<TextEvent>,
) {
    let mut text_event = TextEvent {
        entity: target,
        text: TextBlock {
            text_slices: Vec::new(),
        },
    };

    text_event.text.text_slices.push(TextSlice {
        foreground: 94,
        text: name.clone() + "\n",
        ..Default::default()
    });

    text_event.text.text_slices.push(TextSlice {
        foreground: 7,
        text: description.clone() + "\n",
        ..Default::default()
    });

    text_event_tx.send(text_event);
}

pub fn display_room_to_user(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoom>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    room_query: Query<(&DisplayName, &Description), With<Room>>,
) {
    for event in entity_entered_room_rx.iter() {
        let Ok(controller) = is_controlled_by_query.get(event.entity) else {
            debug!("Couldn't locate a IsControlledByEntity");
            break;
        };

        let (display_name, description) = room_query
            .get(event.room_entity_is_in)
            .expect("Unable to find room entity");

        send_room_description(
            controller.0,
            display_name.0.clone(),
            description.0.clone(),
            &mut text_event_tx,
        );
    }
}

pub fn display_character_entering_room(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoom>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    characters_in_world: Query<&DisplayName, With<Character>>,
    room_info_query: Query<&EntityCollection, With<Room>>,
) {
    for ev in entity_entered_room_rx.iter() {
        if ev.triggered_by != MovementTriggeredBy::UserInput {
            continue;
        }

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

pub fn display_character_logged_into_room(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoom>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    characters_in_world: Query<&DisplayName, With<Character>>,
    room_info_query: Query<&EntityCollection, With<Room>>,
) {
    for ev in entity_entered_room_rx.iter() {
        if ev.triggered_by != MovementTriggeredBy::Login {
            continue;
        }

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
                    &format!("You find yourself disoriented, a blinding bright light filling your vision as your soul leaves suspension. The light pulses as it burns, warm and comforting. After a moment you feel a lurch in your guts, your soul flung from the heart of Ero'ghal and back to the planes it calls home. Almost instantly, you open your eyes and find yourself safe in {} upon the {} plane.", "the Wild Plains", "mortal"),
                ));
            } else {
                text_event_tx.send(TextEvent::new(
                    controlling_entity.0,
                    &format!("Reality bends for a moment as {}'s soul exits suspension, their body appearing, radiating a glowing light that casts no shadows before the world settles back to normal.", display_name.0.clone()),
                ));
            }
        }
    }
}
