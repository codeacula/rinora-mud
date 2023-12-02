use shared::prelude::*;

use crate::events::WelcomeUserEvent;

pub(crate) fn welcome_user_on_login(
    mut welcome_user_rx: EventReader<WelcomeUserEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in welcome_user_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Welcome back!
        
        What would you like to do?
        1. Create a new character
        2. Retire a character",
        ));

        show_prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
