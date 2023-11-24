use shared::prelude::*;

pub fn password_was_provided(
    mut password_provided_rx: EventReader<UserProvidedPasswordEvent>,
    mut please_confirm_password_tx: EventWriter<PleaseConfirmPassword>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in password_provided_rx.read() {
        let mut user_session_data = match query.get_mut(ev.user_entity) {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting user session data: {e}");
                continue;
            }
        };
        user_session_data.status = UserStatus::ConfirmPassword;
        user_session_data.pwd = Some(ev.password.clone());

        please_confirm_password_tx.send(PleaseConfirmPassword(ev.user_entity));
    }
}
