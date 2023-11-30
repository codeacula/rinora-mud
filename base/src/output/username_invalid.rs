use shared::prelude::*;

pub fn username_invalid(
    mut username_invalid_rx: EventReader<UsernameInvalidEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_rx: EventWriter<ShowPromptEvent>,
) {
    for ev in username_invalid_rx.read() {
        text_event_tx.send(TextEvent::from_str(ev.0,
            "{{196}}Sorry, that's an invalid username. {{7}}Usernames must be:\n * Between 3 and 16 characters long\n * Contain only letters, numbers, and underscores\n * Starts with a letter\n"));
        show_prompt_rx.send(ShowPromptEvent(ev.0));
    }
}
