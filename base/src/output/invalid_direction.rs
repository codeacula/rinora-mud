use shared::prelude::*;

pub fn invalid_direction(
    mut invalid_direction_rx: EventReader<InvalidDirectionEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut prompt_event_tx: EventWriter<ShowPromptEvent>,
) {
    for event in invalid_direction_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            event.0,
            "There's no exit in that direction.",
        ));
        prompt_event_tx.send(ShowPromptEvent(event.0));
    }
}
