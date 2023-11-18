use bevy::prelude::*;
use shared::prelude::*;

pub struct HelperPlugin;

fn display_room_debug_info(
    mut entity_entered_room_rx: EventReader<EntityEnteredRoomEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    is_admin_query: Query<&IsAdmin>,
    room_query: Query<&Room>,
) {
    for event in entity_entered_room_rx.read() {
        let Ok(controller) = is_controlled_by_query.get(event.entity) else {
            debug!("Couldn't locate a IsControlledBy");
            break;
        };

        if is_admin_query.get(controller.0).is_err() {
            debug!("Couldn't locate admin tag");
            break;
        }

        let room_going_into = room_query
            .get(event.room_entity_is_in)
            .expect("Unable to find room entity");

        text_event_tx.send(TextEvent::new(
            controller.0,
            &format!("{:?}", room_going_into),
        ));
    }
}

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_room_debug_info.in_set(GameOrderSet::Debug));
    }
}
