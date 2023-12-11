use database::prelude::*;
use shared::prelude::*;

use crate::events::WelcomeUserEvent;

pub(crate) fn show_welcome_menu(
    mut welcome_user_rx: EventReader<WelcomeUserEvent>,
    mut send_text_tx: EventWriter<SendTextToEntityEvent>,
    db_interface: Res<DbInterface>,
    query: Query<&User>,
) {
    for WelcomeUserEvent(entity) in welcome_user_rx.read() {
        let user = match query.get(*entity) {
            Ok(user) => user,
            Err(_) => {
                send_text_tx.send(SendTextToEntityEvent::send_generic_error(*entity));
                error!("User {:?} not found", entity);
                continue;
            }
        };

        let mut output = String::new();

        output.push_str(
            "<<15>>Welcome to RinoraMUD!\n
<<7>>Enter the character name you wish to play, or select an option from below:\n
  Options:
  <<15>>1.<<7>> Create a new character
  <<15>>2.<<7>> Exit\n",
        );

        let characters = match db_interface.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(err) => {
                send_text_tx.send(SendTextToEntityEvent::send_generic_error(*entity));
                error!("Error getting characters for user {}: {}", user.id, err);
                continue;
            }
        };

        if characters.len() > 0 {
            output.push_str("\nYour characters:<<15>>\n");

            for character in characters {
                output.push_str(format!("  {}\n", character.display_name.0).as_str());
            }
        }

        send_text_tx.send(SendTextToEntityEvent::new(*entity, output.as_str()));
    }
}
