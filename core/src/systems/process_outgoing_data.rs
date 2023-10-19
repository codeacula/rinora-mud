use crate::resources::*;
use shared::prelude::*;

pub fn process_outgoing_data(
    outgoing_data_rx: NonSend<OutgoingData>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    for event in outgoing_queue.0.drain(..) {
        if let Err(err) = outgoing_data_rx.0.send(event) {
            warn!("Failed to send outgoing event: {}", err);
        }
    }
}
