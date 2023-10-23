use shared::prelude::*;

/// When a command can't be processed correctly, we send the user a GenericErrorEvent. This sends the text to the user
/// and makes sure they get a prompt
pub fn handle_generic_error(
    mut generic_error_events: EventReader<GenericErrorEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in generic_error_events.iter() {
        text_event_tx.send(TextEvent::send_generic_error(ev.0));
        prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
