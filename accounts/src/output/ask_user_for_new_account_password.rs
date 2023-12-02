use shared::prelude::*;

use crate::events::CreatingNewAccountEvent;

pub(crate) fn ask_user_for_new_account_password(
    mut new_acct_pwd_rx: EventReader<CreatingNewAccountEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in new_acct_pwd_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Looks like you're new here. Please enter a password for your new account:",
        ));
        show_prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
