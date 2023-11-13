use database::prelude::*;
use shared::prelude::*;

pub fn handle_user_login(
    mut events: EventReader<UserLoggedInEvent>,
    mut text_events_tx: EventWriter<TextEvent>,
    mut show_login_tx: EventWriter<ShowLoginScreenEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
    mut query: Query<&mut UserSessionData>,
) {
    for event in events.read() {
        let entity = event.entity;

        let found_user = match db_repo.users.get_by_id(event.id) {
            Ok(user) => user,
            Err(e) => {
                error!("Unable to fetch user after login: {:?}", e);
                text_events_tx.send(TextEvent::send_generic_error(entity));
                continue;
            }
        };

        let Some(user) = found_user else {
            error!("Unable to fetch user after login: No account returned!");
            text_events_tx.send(TextEvent::send_generic_error(entity));
            continue;
        };

        commands.entity(entity).insert(user);

        let Ok(mut session_data) = query.get_mut(entity) else {
            error!("Unable to fetch session data after login!");
            text_events_tx.send(TextEvent::send_generic_error(entity));
            continue;
        };

        session_data.status = UserStatus::LoggedIn;

        show_login_tx.send(ShowLoginScreenEvent(entity));
    }
}
