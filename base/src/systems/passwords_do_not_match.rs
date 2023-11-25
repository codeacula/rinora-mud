use shared::prelude::*;

pub fn passwords_do_not_match(
    mut passwords_no_match_rx: EventReader<ConfirmPasswordDoesNotMatchEvent>,
    mut passwords_dont_match_tx: EventWriter<PasswordsDoNotMatchEvent>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in passwords_no_match_rx.read() {
        passwords_dont_match_tx.send(PasswordsDoNotMatchEvent(ev.0));

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
