use shared::prelude::*;

/// Let the user know that character creation was successful and have the character menu show up
pub fn character_was_created(
    mut main_events: EventReader<CharacterCreatedEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut login_menu_tx: EventWriter<ShowLoginScreenEvent>,
    mut prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in main_events.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Character created! You can now select them from the login screen",
        ));
        login_menu_tx.send(ShowLoginScreenEvent(ev.0));
        prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
