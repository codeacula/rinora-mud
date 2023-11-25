use shared::prelude::*;

pub fn please_confirm_password(
    mut please_confirm_password_rx: EventReader<PleaseConfirmPasswordEvent>,
    mut text_event_rx: EventWriter<TextEvent>,
    mut show_prompt_event_rx: EventWriter<ShowPromptEvent>,
) {
    for ev in please_confirm_password_rx.read() {
        text_event_rx.send(TextEvent::from_str(ev.0, "Please confirm your password.\n"));

        show_prompt_event_rx.send(ShowPromptEvent(ev.0));
    }
}
