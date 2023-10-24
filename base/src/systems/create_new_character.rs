use shared::prelude::*;

pub fn create_new_character(
    mut create_character_rx: EventReader<CreateCharacterEvent>,
    mut character_created_event_tx: EventWriter<CharacterCreatedEvent>,
) {
    for ev in create_character_rx.iter() {}
}
