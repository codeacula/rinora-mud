use shared::prelude::*;

pub fn unable_to_locate_account_move_user(
    mut unable_to_locate_account_rx: EventReader<UnableToLocateAccountEvent>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in unable_to_locate_account_rx.read() {
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
