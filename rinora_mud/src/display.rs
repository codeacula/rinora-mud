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
            .get(event.room)
            .expect("Unable to find room entity");

        send_room_description(
            controller.0,
            display_name.0.clone(),
            description.0.clone(),
            &mut text_event_tx,
        );
    }
}
