use database::prelude::*;
use shared::prelude::*;

use crate::events::WelcomeUserEvent;

pub(crate) fn show_welcome_menu(
    mut welcome_user_rx: EventReader<WelcomeUserEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_event_tx: EventWriter<ShowPromptEvent>,
    db_interface: Res<DbInterface>,
    query: Query<&User>,
) {
    for WelcomeUserEvent(entity) in welcome_user_rx.read() {
        let user = match query.get(*entity) {
            Ok(user) => user,
            Err(_) => {
                text_event_tx.send(TextEvent::send_generic_error(*entity));
                error!("User {:?} not found", entity);
                continue;
            }
        };

        let mut output = String::new();

        output.push_str(
            "{{15}}Welcome to RinoraMUD!\n
{{7}}Either enter the character name you wish to play, or would like to create.",
        );

        let characters = match db_interface.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(err) => {
                text_event_tx.send(TextEvent::send_generic_error(*entity));
                error!("Error getting characters for user {}: {}", user.id, err);
                continue;
            }
        };

        if characters.len() > 0 {
            output.push_str("\nYour characters:{{15}}\n");

            for character in characters {
                output.push_str(format!("  {}\n", character.display_name.0).as_str());
            }
        }

        text_event_tx.send(TextEvent::from_str(*entity, output.as_str()));
        show_prompt_event_tx.send(ShowPromptEvent(*entity));
    }
}
