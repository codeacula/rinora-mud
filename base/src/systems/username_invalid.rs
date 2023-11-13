use shared::prelude::*;

pub fn username_invalid(
    mut username_invalid_rx: EventReader<UsernameInvalidEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_rx: EventWriter<ShowPromptEvent>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in username_invalid_rx.read() {
        let mut user_session_data = match query.get_mut(ev.0) {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting user session data: {e}");
                continue;
            }
        };
        user_session_data.status = UserStatus::NeedUsername;

        text_event_tx.send(TextEvent::from_str(ev.0, 
            "{{196}}Sorry, that's an invalid username. {{7}}Usernames must be:\n * Between 3 and 16 characters long\n * Contain only letters, numbers, and underscores\n * Starts with a letter"));
        show_prompt_rx.send(ShowPromptEvent(ev.0));
    }
}
