use shared::prelude::*;

pub fn invalid_direction(
    mut invalid_direction_rx: EventReader<InvalidDirectionEvent>,
    text_event_tx: EventWriter<TextEvent>,
    prompt_event_tx: EventWriter<ShowPromptEvent>,
) {
    for event in invalid_direction_rx.read() {
        text_event_tx.send(TextEvent {
            text: format!("You can't go {}.", event.direction),
        });
        prompt_event_tx.send(ShowPromptEvent(event.0));
    }
}
