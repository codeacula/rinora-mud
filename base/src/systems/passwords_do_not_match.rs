use shared::prelude::*;

pub fn passwords_do_not_match(
    mut passwords_no_match_rx: EventReader<ConfirmPasswordDoesNotMatchEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_rx: EventWriter<ShowPromptEvent>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in passwords_no_match_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Looks like your passwords don't match. Let's try again. Please enter a password:",
        ));
        show_prompt_rx.send(ShowPromptEvent(ev.0));

        let mut user_session_data = match query.get_mut(ev.0) {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting user session data: {e}");
                continue;
            }
        };
        user_session_data.status = UserStatus::CreatePassword;
    }
}
