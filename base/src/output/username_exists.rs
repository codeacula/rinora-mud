use shared::prelude::*;

pub fn username_exists(
    mut username_exists_rx: EventReader<UsernameExistsEvent>,
    mut text_event_rx: EventWriter<TextEvent>,
    mut show_prompt_event_rx: EventWriter<ShowPromptEvent>,
) {
    for ev in username_exists_rx.read() {
        text_event_rx.send(TextEvent::from_str(
            ev.0,
            "Account found! What's your password?",
        ));

        show_prompt_event_rx.send(ShowPromptEvent(ev.0));
    }
}
