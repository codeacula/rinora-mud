use shared::prelude::*;

pub fn user_account_created(
    mut user_account_created_rx: EventReader<UserAccountCreatedEvent>,
    mut text_event_writer_tx: EventWriter<TextEvent>,
    mut show_prompt_writer_tx: EventWriter<ShowPromptEvent>,
) {
    for ev in user_account_created_rx.read() {
        text_event_writer_tx.send(TextEvent::from_str(
            ev.0,
            "\nYour account was created. {{10}} Welcome to RinoraMUD!\n\n",
        ));
        show_prompt_writer_tx.send(ShowPromptEvent(ev.0));
    }
}
