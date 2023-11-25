use shared::prelude::*;

use crate::events::NewConnectionEvent;

pub fn provide_username(
    mut new_connection_event_rx: EventReader<NewConnectionEvent>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
    mut show_prompt_ev: EventWriter<ShowPromptEvent>,
) {
    for ev in new_connection_event_rx.read() {
        ev_outgoing_text_events.send(TextEvent::from_str(
            ev.entity,
            "Please provide your username.\n",
        ));
        show_prompt_ev.send(ShowPromptEvent(ev.entity));
    }
}
