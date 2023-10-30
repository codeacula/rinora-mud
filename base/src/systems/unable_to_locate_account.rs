use shared::prelude::*;

pub fn unable_to_locate_account(
    mut unable_to_locate_account_rx: EventReader<UnableToLocateAccountEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_rx: EventWriter<ShowPromptEvent>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in unable_to_locate_account_rx.iter() {
        let mut user_session_data = match query.get_mut(ev.0) {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting user session data: {e}");
                continue;
            }
        };
        user_session_data.status = UserStatus::NeedUsername;

        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Those credentials don't seem to match. Try again.",
        ));
        show_prompt_rx.send(ShowPromptEvent(ev.0));
    }
}
