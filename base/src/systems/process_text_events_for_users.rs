use crate::OutgoingQueue;
use bevy::prelude::*;
use shared::prelude::*;

fn build_color_code(slice: &TextSlice) -> String {
    //"\u{1b}[1;31mtest\u{1b}[0ming\n"
    let mut code_bits: Vec<String> = Vec::new();

    code_bits.push(format!("38:5:{}", slice.foreground));
    code_bits.push(format!("48:5:{}", slice.background));

    format!("\u{1b}[{}m", code_bits.join(";"))
}

pub fn process_text_events_for_users(
    query: Query<&UserSessionData>,
    mut incoming_text_events: EventReader<TextEvent>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    for text_event in incoming_text_events.iter() {
        let user = match query.get(text_event.entity) {
            // Is an entity that isn't a user, like an NPC
            Err(_) => continue,
            Ok(user) => user,
        };

        let mut outgoing_string = String::from("");

        for slice in text_event.text.text_slices.iter() {
            outgoing_string.push_str(&format!("{}{}", build_color_code(slice), slice.text))
        }

        // Reset formatting and add a newline
        outgoing_string.push_str("\u{1b}[0m");
        outgoing_queue.send_str(user.connection, &format!("{}\n", outgoing_string.trim()));
    }
}
