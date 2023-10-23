use shared::prelude::*;

/// Tells the user the character name they provided is already in use
pub fn display_character_exists(
    mut main_events: EventReader<CharacterExistsEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in main_events.iter() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "That character already exists. Please try a different name.",
        ));
        prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
