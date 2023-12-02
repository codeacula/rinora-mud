use shared::prelude::*;

use crate::events::WelcomeUserEvent;

pub(crate) fn show_welcome_menu(
    mut welcome_user_rx: EventReader<WelcomeUserEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_event_tx: EventWriter<ShowPromptEvent>,
) {
    for WelcomeUserEvent(entity) in welcome_user_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            *entity,
"{{15}}Welcome to RinoraMUD!\n{{7}}Either provide the character you want to play, or select an option from below:
  1. Create a new character",
        ));

        show_prompt_event_tx.send(ShowPromptEvent(*entity));
    }
}
