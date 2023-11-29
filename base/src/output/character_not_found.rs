use shared::prelude::*;

pub fn character_not_found(
    mut character_not_found_rx: EventReader<CharacterNotFoundEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in character_not_found_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Character not found. Please try again.\n",
        ));
        show_prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
