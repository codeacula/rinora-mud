use bevy::prelude::*;
use shared::prelude::*;

pub fn display_room_to_user(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoom>,
    mut text_event_tx: EventWriter<TextEvent>,
    characters: Res<CharacterMap>,
    room_query: Query<&Room>,
    user_query: Query<&User>,
) {
    for event in entity_entered_room_rx.iter() {
        if let Ok(room) = room_query.get(event.room) {
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
}
