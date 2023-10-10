use bevy::prelude::*;
use shared::prelude::*;

fn send_room_description(target: Entity, room: &Room, text_event_tx: &mut EventWriter<TextEvent>) {
    let mut text_event = TextEvent {
        entity: target,
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

pub fn display_room_to_user(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoom>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    room_query: Query<&Room>,
) {
    for event in entity_entered_room_rx.iter() {
        debug!("Entity entered room: {:?}", is_controlled_by_query);
        let Ok(controller) = is_controlled_by_query.get(event.entity) else {
            debug!("Couldn't locate a IsControlledByEntity");
            break;
        };
        
        let room_going_into = room_query
            .get(event.room)
            .expect("Unable to find room entity");

        info!("32 {:?}", event);
        info!("42 {:?}", room_going_into);
        send_room_description(controller.0, room_going_into, &mut text_event_tx);
    }
}
