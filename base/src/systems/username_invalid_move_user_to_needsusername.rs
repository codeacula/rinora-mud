use shared::prelude::*;

pub fn username_invalid_move_user_to_needs_username(
    mut username_invalid_rx: EventReader<UsernameInvalidEvent>,
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
    }
}
