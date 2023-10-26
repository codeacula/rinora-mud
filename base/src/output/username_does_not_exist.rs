use shared::prelude::*;

pub fn username_does_not_exists(
    mut username_not_exists_rx: EventReader<UsernameDoesNotExistEvent>,
    mut text_event_rx: EventWriter<TextEvent>,
    mut show_prompt_event_rx: EventWriter<ShowPromptEvent>,
) {
    for ev in username_not_exists_rx.iter() {
        text_event_rx.send(TextEvent::from_str(
            ev.0,
            "{{11}}It looks like this is your first time here! {{7}}What would you like your password to be?",
        ));

        show_prompt_event_rx.send(ShowPromptEvent(ev.0));
    }
}
