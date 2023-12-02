use shared::prelude::*;

use crate::events::ConfirmingPasswordEvent;

pub(crate) fn confirm_account_password(
    mut new_acct_pwd_rx: EventReader<ConfirmingPasswordEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in new_acct_pwd_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "I've located your account. What is your password?",
        ));
        show_prompt_tx.send(ShowPromptEvent(ev.0));
    }
}
