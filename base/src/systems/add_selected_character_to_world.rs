use database::prelude::DbInterface;
use shared::prelude::*;

pub fn add_selected_character_to_world(
    mut character_selected_rx: EventReader<CharacterSelectedEvent>,
    db_repo: Res<DbInterface>,
    mut entity_enters_world_tx: EventWriter<EntityEnteredWorldEvent>,
    mut entity_enters_room_tx: EventWriter<EntityEnteredRoomEvent>,
    mut commands: Commands,
    room_map: Res<RoomMap>,
) {
    for ev in character_selected_rx.read() {
        let character = match db_repo.characters.get_character_by_name(&ev.name) {
            Ok(character) => match character {
                Some(character) => character,
                None => {
                    error!("Character not found: {}", ev.name);
                    continue;
                }
            },
            Err(e) => {
                error!("Error getting character: {:?}", e);
                continue;
            }
        };

        let location_id = character.location.0;
        let mut character_ent = commands.spawn(character);
        let room = *room_map.0.get(&location_id).unwrap();

        character_ent.insert(IsControlledBy(ev.user_entity));

        entity_enters_world_tx.send(EntityEnteredWorldEvent {
            entity: character_ent.id(),
            room_entity_is_in: room,
            triggered_by: MovementTriggeredBy::Login,
        });

        entity_enters_room_tx.send(EntityEnteredRoomEvent {
            entity: character_ent.id(),
            room_entity_is_in: room,
            triggered_by: MovementTriggeredBy::Login,
        });
    }
}
