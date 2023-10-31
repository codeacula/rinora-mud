use database::prelude::*;
use shared::prelude::*;

pub fn show_login_screen(
    mut main_events: EventReader<ShowLoginScreenEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_tx: EventWriter<ShowPromptEvent>,
    mut generic_tx: EventWriter<GenericErrorEvent>,
    db_repo: Res<DbInterface>,
    query: Query<(&User, &UserSessionData)>,
) {
    for ev in main_events.iter() {
        let entity = ev.0;

        let Ok((user, user_sesh)) = query.get(entity) else {
            info!("Entity had no associated user information: {entity:?}");
            generic_tx.send(GenericErrorEvent(entity));
            continue;
        };

        let characters = match db_repo.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!("Unable to fetch user's characters after creating a character: {e:?}");
                generic_tx.send(GenericErrorEvent(entity));
                continue;
            }
        };

        let mut greeting = String::from("Your options:\n\n");

        greeting.push_str("  [{{15}}1{{7}}]: Create Character\n");

        if characters.is_empty() {
            greeting.push_str("You currently have no characters.\n\n")
        } else {
            greeting.push_str("Your characters are:\n");

            for character in characters {
                greeting.push_str(&format!("  {}\n", character.display_name.0));
            }
        }

        greeting.push_str("\nSend a number command or which character you want to play.");

        text_event_tx.send(TextEvent::new(entity, &greeting));
        show_prompt_tx.send(ShowPromptEvent(entity));
    }
}
