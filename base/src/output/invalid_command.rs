use shared::prelude::*;

use crate::events::InvalidCommandEvent;

pub fn invalid_command(
    mut invalid_command_rx: EventReader<InvalidCommandEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut prompt_event_tx: EventWriter<ShowPromptEvent>,
) {
    for event in invalid_command_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            event.0,
            "I don't understand what you mean.\n",
        ));
        prompt_event_tx.send(ShowPromptEvent(event.0));
    }
}
