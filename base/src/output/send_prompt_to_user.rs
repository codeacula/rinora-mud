use shared::prelude::*;

pub fn send_prompt_to_user(
    mut show_prompt_rx: EventReader<ShowPromptEvent>,
    query: Query<&UserSessionData>,
    mut text_event_tx: EventWriter<TextEvent>,
) {
    let mut sent_map: HashMap<Entity, bool> = HashMap::new();

    for ev in show_prompt_rx.read() {
        let entity = ev.0;

        let Ok(session_data) = query.get(entity) else {
            continue;
        };

        if let Some(sent) = sent_map.get_mut(&entity) {
            if *sent {
                continue;
            }
            *sent = true;
        } else {
            sent_map.insert(entity, true);
        }

        if session_data.status == UserStatus::InGame {
            text_event_tx.send(TextEvent::from_str(entity, "-"));
        } else {
            let mut sent_event = TextEvent::from_str(entity, "> \n");
            sent_event.add_newline = false;
            text_event_tx.send(sent_event);
        }
    }
}
