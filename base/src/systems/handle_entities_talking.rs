use bevy::ecs::prelude::*;
use shared::prelude::*;

pub fn handle_entities_talking(
    mut speak_event_rx: EventReader<SpeakEvent>,
    collection_query: Query<&EntityCollection>,
    mut heard_event_tx: EventWriter<HeardEvent>,
) {
    for ev in speak_event_rx.read() {
        let entity_collection = match collection_query.get(ev.room) {
            Ok(entity_collection) => entity_collection,
            Err(_) => {
                continue;
            }
        };

        for entity_in_room in entity_collection.0.iter() {
            let heard_event = HeardEvent {
                listener: *entity_in_room,
                speaker: ev.speaker,
                target: ev.target,
                text: ev.text.clone(),
            };

            heard_event_tx.send(heard_event);
        }
    }
}
