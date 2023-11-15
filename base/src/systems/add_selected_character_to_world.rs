use database::prelude::DbInterface;
use shared::prelude::*;

pub fn add_selected_character_to_world(
    mut character_selected_rx: EventReader<CharacterSelectedEvent>,
    db_repo: Res<DbInterface>,
    mut move_entity_to_room_rx: EventWriter<MoveEntityToRoom>,
    mut commands: Commands,
    room_map: Res<RoomMap>,
    mut query: Query<&mut UserSessionData>,
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

        let location_id = character.location.location_id;
        let mut character_ent = commands.spawn(character);
        let room = *room_map.0.get(&location_id).unwrap();

        character_ent.insert(IsControlledBy(ev.user_entity));

        move_entity_to_room_rx.send(MoveEntityToRoom {
            entity: character_ent.id(),
            room,
        });

        let mut user_sesh = query.get_mut(ev.user_entity).unwrap();
        user_sesh.status = UserStatus::InGame;
        user_sesh.controlling_entity = Some(character_ent.id());
    }
}
