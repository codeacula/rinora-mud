use std::sync::mpsc::Sender;

use shared::prelude::*;

use crate::{NetworkEventType, OutgoingEvent};

fn build_color_code(slice: &TextSlice) -> String {
    //"\u{1b}[1;31mtest\u{1b}[0ming\n"
    let mut code_bits: Vec<String> = Vec::new();

    code_bits.push(format!("38:5:{}", slice.foreground));
    code_bits.push(format!("48:5:{}", slice.background));

    format!("\u{1b}[{}m", code_bits.join(";"))
}

pub(crate) fn process_text_events(
    mut text_event_rx: EventReader<TextEvent>,
    query: Query<&UserSessionData>,
    outgoing_event_tx: NonSend<Sender<OutgoingEvent>>,
) {
    for text_event in text_event_rx.read() {
        let user_sesh = match query.get(text_event.entity) {
            Ok(user_sesh) => user_sesh,
            Err(_) => {
                error!("No user session found for entity! {:?}", text_event.entity);
                continue;
            }
        };

        let mut outgoing_string = String::from("");

        for slice in text_event.text.text_slices.iter() {
            outgoing_string.push_str(&format!("{}{}", build_color_code(slice), slice.text))
        }

        if !outgoing_string.ends_with("\n") {
            outgoing_string.push_str("\n");
        }

        // Reset formatting and add a newline
        outgoing_string.push_str("\u{1b}[0m");
        outgoing_string = outgoing_string.trim().to_string();

        let outgoing_bytes = outgoing_string.into_bytes();

        outgoing_event_tx
            .send(OutgoingEvent {
                id: user_sesh.connection,
                data: Some(outgoing_bytes),
                event_type: NetworkEventType::Text,
            })
            .expect("Failed to send outgoing event!");
    }
}
