use bevy::prelude::*;
use shared::prelude::*;

pub fn send_heard_to_user(
    mut heard_event_rx: EventReader<HeardEvent>,
    display_name_query: Query<&DisplayName>,
    mut send_text_tx: EventWriter<SendTextToEntityEvent>,
    controlled_by_query: Query<&IsControlledBy>,
) {
    for ev in heard_event_rx.read() {
        let user_to_send_to = match controlled_by_query.get(ev.listener) {
            Ok(user_to_send_to) => user_to_send_to,
            Err(_) => {
                continue;
            }
        };

        let mut speaker = String::from("An unknown voice");
        let mut to = String::from("");

        if let Ok(display_name) = display_name_query.get(ev.speaker) {
            speaker = display_name.0.clone();
        }

        if let Some(target_entity) = ev.target {
            if target_entity == ev.listener {
                to = String::from(" you");
            } else if let Ok(display_name) = display_name_query.get(target_entity) {
                to = format!(" to {}", display_name.0);
            }
        }

        let text_to_send = format!("{} says{}, \"{}\"", speaker, to, ev.text);

        send_text_tx.send(SendTextToEntityEvent {
            entity: user_to_send_to.0,
            text: text_to_send,
        });
    }
}
