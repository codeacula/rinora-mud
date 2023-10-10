use bevy::prelude::*;
use shared::prelude::*;

fn send_being_entered_description() {}

fn send_room_description() {}

pub fn display_room_to_user(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoom>,
    mut text_event_tx: EventWriter<TextEvent>,
    characters: Res<CharacterMap>,
    room_query: Query<&Room>,
    being_query: Query<&Being, &IsControlledBy>,
) {
    for event in entity_entered_room_rx.iter() {
        let room = room_query
            .get(event.room)
            .expect("Unale to find room entity");

        for being in room.entities.iter() {
            if being == &event.entity {
                todo!("Show the room to the user!");
            } else {
                todo!("Show the entity entering to the character.");
            }
        }

        let mut text_event = TextEvent {
            entity: event.entity,
            text: TextBlock {
                text_slices: Vec::new(),
            },
        };

        text_event.text.text_slices.push(TextSlice {
            foreground: 94,
            text: room.name.clone() + "\n",
            ..Default::default()
        });

        text_event.text.text_slices.push(TextSlice {
            foreground: 7,
            text: room.description.clone() + "\n",
            ..Default::default()
        });

        text_event_tx.send(text_event);
    }
}
