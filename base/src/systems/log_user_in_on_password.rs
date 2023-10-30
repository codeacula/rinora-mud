use database::prelude::*;
use shared::prelude::*;

pub fn log_user_in_on_password(
    mut user_confirmed_password_rx: EventReader<UserConfirmedPasswordEvent>,
    mut show_login_screen_event: EventWriter<ShowLoginScreenEvent>,
    mut query: Query<&mut UserSessionData>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for ev in user_confirmed_password_rx.iter() {
        let mut user_sesh = match query.get_mut(ev.0) {
            Ok(val) => val,
            Err(err) => {
                error!("Failed to get user session data: {err:?}");
                continue;
            }
        };

        let Some(password) = user_sesh.pwd.clone() else {
            error!("User session data did not have a password");
            continue;
        };

        info!(
            "Attempting to create user: {} with password: {password}",
            user_sesh.username
        );

        let user = match db_repo.users.create_user(&user_sesh.username, &password) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to create the user: {e:?}");
                continue;
            }
        };
        user_sesh.status = UserStatus::LoggedIn;

        commands.entity(ev.0).insert(user);
        show_login_screen_event.send(ShowLoginScreenEvent(ev.0));
    }
}
