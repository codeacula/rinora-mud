use database::prelude::*;
use shared::prelude::*;

pub fn handle_user_login(
    mut query: Query<Entity>,
    mut events: EventReader<UserLoggedInEvent>,
    mut text_events_tx: EventWriter<TextEvent>,
    mut show_login_tx: EventWriter<ShowLoginScreenEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let entity = query.get_mut(event.entity).unwrap();

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

        show_login_tx.send(ShowLoginScreenEvent(entity));
    }
}
