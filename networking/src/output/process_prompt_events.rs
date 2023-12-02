use std::sync::mpsc::Sender;

use shared::prelude::*;

use crate::{
    constants::{GA, IAC},
    NetworkEventType, OutgoingEvent,
};

pub(crate) fn process_prompt_events(
    mut show_prompt_rx: EventReader<ShowPromptEvent>,
    query: Query<&UserSessionData>,
    outgoing_event_tx: NonSend<Sender<OutgoingEvent>>,
) {
    let mut processed_ids = HashMap::<Entity, bool>::new();
    for prompt_event in show_prompt_rx.read() {
        if processed_ids.contains_key(&prompt_event.0) {
            continue;
        }

        processed_ids.insert(prompt_event.0, true);

        let user_sesh = match query.get(prompt_event.0) {
            Ok(user_sesh) => user_sesh,
            Err(_) => {
                error!("No user session found for entity! {:?}", prompt_event.0);
                continue;
            }
        };

        // We only send the ICA GA because other things should handle displaying the actual prompt
        outgoing_event_tx
            .send(OutgoingEvent {
                id: user_sesh.connection,
                data: Some(vec![IAC, GA]),
                event_type: NetworkEventType::Gmcp,
            })
            .expect("Failed to send outgoing event!");
    }
}
