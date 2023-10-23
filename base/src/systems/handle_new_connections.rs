use crate::events::*;
use shared::prelude::*;

/// When someone first connects
pub fn handle_new_connections(
    mut ev_new_connection: EventReader<NewConnectionEvent>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
) {
    for ev in ev_new_connection.iter() {
        ev_outgoing_text_events.send(TextEvent::from_str(
            ev.entity,
            "Please provide your username.",
        ));
    }
}
