use shared::prelude::*;

pub fn unable_to_locate_account(
    mut unable_to_locate_account_rx: EventReader<UnableToLocateAccountEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_rx: EventWriter<ShowPromptEvent>,
) {
    for ev in unable_to_locate_account_rx.read() {
        text_event_tx.send(TextEvent::from_str(
            ev.0,
            "Those credentials don't seem to match. Try again.\n",
        ));
        show_prompt_rx.send(ShowPromptEvent(ev.0));
    }
}
