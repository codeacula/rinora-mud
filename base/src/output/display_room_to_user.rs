use shared::prelude::*;

use crate::{events::ShowRoomToBeing, helpers::*};

pub fn display_room_to_user(
    mut show_room_to_being_rx: EventReader<ShowRoomToBeing>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    room_query: Query<(&DisplayName, &Description, &Exits, &Room)>,
    exit_query: Query<&Exit>,
    mut send_prompt_tx: EventWriter<ShowPromptEvent>,
    mut gmcp_data_tx: EventWriter<SendGmcpData>,
) {
    for event in show_room_to_being_rx.read() {
        let Ok(controller) = is_controlled_by_query.get(event.entity) else {
            debug!("Couldn't locate a IsControlledByEntity");
            break;
        };

        let (display_name, description, exits, room) = room_query
            .get(event.room)
            .expect("Unable to find room entity");

        send_room_description(
            controller.0,
            &display_name.0,
            &description.0,
            exits,
            &exit_query,
            &mut text_event_tx,
        );

        // Build the gmcp data
        send_room_gmcp(&mut gmcp_data_tx, controller, room.room_id, &display_name.0);

        send_prompt_tx.send(ShowPromptEvent(controller.0));
    }
}
