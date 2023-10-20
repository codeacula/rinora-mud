use shared::prelude::*;

pub fn show_prompt(
    mut show_prompt_rx: EventReader<ShowPrompt>,
    query: Query<&UserSessionData>,
    mut text_event_tx: EventWriter<TextEvent>,
) {
    for ev in show_prompt_rx.iter() {
        let entity = ev.0;

        let Ok(session_data) = query.get(entity) else {
            continue;
        };

        if session_data.status == UserStatus::LoggedIn {
            text_event_tx.send(TextEvent::from_str(entity, "> "));
        } else {
            text_event_tx.send(TextEvent::from_str(entity, "-"));
        }
    }
}
