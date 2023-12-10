use std::sync::mpsc::Sender;

use shared::prelude::*;

use crate::{
    constants::{GA, IAC},
    NetworkEventType, OutgoingEvent,
};

pub(crate) fn process_prompt_events(
    mut send_ga_rx: EventReader<SendGoAheadEvent>,
    query: Query<&UserSessionData>,
    outgoing_event_tx: NonSend<Sender<OutgoingEvent>>,
) {
    for event in send_ga_rx.read() {
        let user_sesh = match query.get(event.0) {
            Ok(user_sesh) => user_sesh,
            Err(_) => {
                error!("No user session found for entity! {:?}", event.0);
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
