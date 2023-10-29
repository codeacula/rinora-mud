use shared::prelude::*;

pub fn password_was_provided(
    mut password_provided_rx: EventReader<UserProvidedPasswordEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_rx: EventWriter<ShowPromptEvent>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in password_provided_rx.iter() {
        let mut user_session_data = match query.get_mut(ev.user_entity) {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting user session data: {e}");
                continue;
            }
        };
        user_session_data.status = UserStatus::ConfirmPassword;
        user_session_data.pwd = Some(ev.password.clone());
        text_event_tx.send(TextEvent::from_str(
            ev.user_entity,
            "Please confirm your password:",
        ));
        show_prompt_rx.send(ShowPromptEvent(ev.user_entity));
    }
}
