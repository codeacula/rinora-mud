use database::prelude::*;
use shared::prelude::*;

pub fn create_new_user(
    mut user_confirmed_password_rx: EventReader<UserConfirmedPasswordEvent>,
    mut show_login_screen_event: EventWriter<ShowLoginScreenEvent>,
    mut query: Query<&mut UserSessionData>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
    mut user_created_tx: EventWriter<UserCreatedEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
) {
    for ev in user_confirmed_password_rx.iter() {
        let mut user_sesh = match query.get_mut(ev.0) {
            Ok(val) => val,
            Err(err) => {
                error!("Failed to get user session data: {err:?}");
                continue;
            }
        };

        let password = match user_sesh.pwd.clone() {
            Some(val) => val,
            None => {
                error!("User session data did not have a password");
                continue;
            }
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

        commands.entity(ev.0).insert(user);
        user_created_tx.send(UserCreatedEvent(ev.0));

        user_sesh.status = UserStatus::LoggedIn;

        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "\nYour account was created. {{10}} Welcome to RinoraMUD!\n\n",
        ));
        show_login_screen_event.send(ShowLoginScreenEvent(ev.0));
    }
}
