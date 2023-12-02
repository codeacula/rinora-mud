use shared::prelude::*;

pub(crate) fn display_prompt(
    mut send_prompt_rx: EventReader<ShowPromptEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    query: Query<&UserSessionData>,
) {
    for ev in send_prompt_rx.read() {
        let user_sesh = match query.get(ev.0) {
            Ok(sesh) => sesh,
            Err(_) => {
                error!("User session not found for prompt event");
                continue;
            }
        };

        if user_sesh.entity_they_are_controlling.is_none() {
            text_event_tx.send(TextEvent::from_str(ev.0, ">"));
            continue;
        }

        // Do prompt stuff here
        text_event_tx.send(TextEvent::from_str(ev.0, "H:100 M:100 -"));
    }
}
