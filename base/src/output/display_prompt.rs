use shared::prelude::*;

pub(crate) fn display_prompt(
    mut send_prompt_rx: EventReader<ShowPromptEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    query: Query<&UserSessionData>,
    mut send_ga_tx: EventWriter<SendGoAheadEvent>,
) {
    let mut processed_ids = HashMap::<Entity, bool>::new();
    for ev in send_prompt_rx.read() {
        if processed_ids.contains_key(&ev.0) {
            continue;
        }
        processed_ids.insert(ev.0, true);

        let user_sesh = match query.get(ev.0) {
            Ok(sesh) => sesh,
            Err(_) => {
                error!("User session not found for prompt event");
                continue;
            }
        };

        if user_sesh.entity_they_are_controlling.is_none() {
            text_event_tx.send(TextEvent::from_str(ev.0, ">"));
            send_ga_tx.send(SendGoAheadEvent(ev.0));
            continue;
        }

        // Do prompt stuff here
        text_event_tx.send(TextEvent::from_str(ev.0, "H:100 M:100 -"));
        send_ga_tx.send(SendGoAheadEvent(ev.0));
    }
}
