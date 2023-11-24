use shared::prelude::*;

pub fn passwords_dont_match(
    mut passwords_no_match_rx: EventReader<ConfirmPasswordDoesNotMatchEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_rx: EventWriter<ShowPromptEvent>,
) {
    for ev in passwords_no_match_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Looks like your passwords don't match. Let's try again. Please enter a password:\n",
        ));
        show_prompt_rx.send(ShowPromptEvent(ev.0));
    }
}
