use shared::prelude::*;

pub fn prompt_for_character_name(
    mut main_events: EventReader<PromptUserForCharacterName>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in main_events.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Please provide a character name. Character names can only be 15 characters long and only contain letters.",
        ));
        prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
