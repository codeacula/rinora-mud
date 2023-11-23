use shared::prelude::*;

use crate::helpers::*;

pub fn display_room_to_entity(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoomEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    room_query: Query<(&DisplayName, &Description, &Exits), With<Room>>,
    exit_query: Query<&Exit>,
    mut send_prompt_tx: EventWriter<ShowPromptEvent>,
    mut gmcp_data_tx: EventWriter<SendGmcpData>,
) {
    for event in entity_entered_room_rx.read() {
        let Ok(controller) = is_controlled_by_query.get(event.entity) else {
            debug!("Couldn't locate a IsControlledByEntity");
            break;
        };

        let (display_name, description, exits) = room_query
            .get(event.room_entity_is_in)
            .expect("Unable to find room entity");

        send_room_description(
            controller.0,
            &display_name.0,
            &description.0,
            exits,
            &exit_query,
            &mut text_event_tx,
        );

        // Build the gmcp data:=
        send_room_gmcp(&gmcp_data_tx, &controller);

        send_prompt_tx.send(ShowPromptEvent(controller.0));
    }
}
