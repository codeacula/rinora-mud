use shared::prelude::*;

use crate::{events::UserConnectedEvent, ConnectionToEntityMap};

pub(crate) fn ask_for_username(
    mut user_connected_rx: EventReader<UserConnectedEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    mut prompt_event_tx: EventWriter<ShowPromptEvent>,
    user_map: Res<ConnectionToEntityMap>,
) {
    for ev in user_connected_rx.read() {
        let entity = match user_map.0.get(&ev.0) {
            Some(entity) => entity,
            None => {
                error!("No entity found for connection! {:?}", ev.0);
                continue;
            }
        };

        text_event_tx.send(TextEvent::from_str(*entity, "Please provide a username."));
        prompt_event_tx.send(ShowPromptEvent(*entity));
    }
}
