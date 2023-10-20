use database::prelude::*;
use shared::prelude::*;

use crate::output::get_login_screen::*;

pub fn handle_user_login(
    mut query: Query<Entity>,
    mut events: EventReader<UserLoggedIn>,
    mut text_events_tx: EventWriter<TextEvent>,
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

        let characters = match db_repo.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!("Unable to fetch user's characters at login: {:?}", e);
                text_events_tx.send(TextEvent::from_str(
                    entity,
                    "There was an issue fetching your characters. Please disconnect and try again.",
                ));
                continue;
            }
        };

        text_events_tx.send(TextEvent::new(entity, &get_login_screen(&characters)));
        commands.entity(entity).insert(user);
    }
}
