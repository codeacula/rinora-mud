use database::prelude::*;
use shared::prelude::*;

pub fn check_username_and_transition_user(
    mut username_provided_rx: EventReader<UsernameProvidedEvent>,
    db_repo: Res<DbInterface>,
    mut generic_error_rx: EventWriter<GenericErrorEvent>,
    mut username_does_not_exist_rx: EventWriter<UsernameDoesNotExistEvent>,
    mut username_exists_rx: EventWriter<UsernameExistsEvent>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in username_provided_rx.iter() {
        // Look if the username exists in the db
        let username_exists = match db_repo.users.does_user_exist(&ev.username) {
            Ok(val) => val,
            Err(e) => {
                error!("Error checking if username exists: {e}");
                generic_error_rx.send(GenericErrorEvent(ev.user_entity));
                continue;
            }
        };

        let mut user_session_data = match query.get_mut(ev.user_entity) {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting user session data: {e}");
                generic_error_rx.send(GenericErrorEvent(ev.user_entity));
                continue;
            }
        };

        if username_exists {
            username_exists_rx.send(UsernameExistsEvent(ev.user_entity));
            user_session_data.status = UserStatus::NeedPassword;
        } else {
            username_does_not_exist_rx.send(UsernameDoesNotExistEvent(ev.user_entity));
            user_session_data.status = UserStatus::CreatePassword;
        }
    }
}
