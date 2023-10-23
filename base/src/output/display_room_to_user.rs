use shared::prelude::*;

use crate::helpers::*;

pub fn display_room_to_user(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoomEvent>,
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
