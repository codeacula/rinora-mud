use shared::prelude::*;

pub(crate) fn send_text_to_entity(
    mut send_text_to_entity_rx: EventReader<SendTextToEntityEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut show_prompt_tx: EventWriter<ShowPromptEvent>,
    is_controlled_by_query: Query<&IsControlledBy>,
    is_user_query: Query<Entity, With<User>>,
) {
    for ev in send_text_to_entity_rx.read() {
        if is_user_query.get(ev.entity).is_ok() {
            text_event_tx.send(TextEvent::from_str(ev.entity, &ev.text));
            show_prompt_tx.send(ShowPromptEvent(ev.entity));
            continue;
        }

        let controlling_entity = match is_controlled_by_query.get(ev.entity) {
            Ok(controlling_entity) => controlling_entity,
            Err(_) => {
                continue;
            }
        };

        let entity = controlling_entity.0;

        text_event_tx.send(TextEvent::from_str(entity, &ev.text));
        show_prompt_tx.send(ShowPromptEvent(entity));
    }
}
