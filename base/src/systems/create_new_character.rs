use database::prelude::*;
use shared::prelude::*;

pub fn create_new_character(
    mut create_character_rx: EventReader<CreateCharacterEvent>,
    mut query: Query<(Entity, &User, &mut UserSessionData)>,
    db_handle: Res<DbInterface>,
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut character_created_tx: EventWriter<CharacterCreatedEvent>,
    mut move_entity_to_room: EventWriter<MoveEntityToRoom>,
    room_map: Res<RoomMap>,
) {
    for ev in create_character_rx.read() {
        let Ok((user_entity, user, mut user_session_data)) = query.get_mut(ev.user_entity) else {
            error!("User session data not found for user {:?}", ev.user_entity);
            continue;
        };

        let creation_result =
            match db_handle
                .characters
                .create_character(&ev.name, settings.default_room, user)
            {
                Ok(character) => character,
                Err(e) => {
                    error!("Error creating character: {:?}", e);
                    continue;
                }
            };

        let mut ent_commands = commands.spawn(creation_result);
        let character_entity = ent_commands.id();

        ent_commands.insert(IsControlledBy(user_entity));

        user_session_data.entity_they_are_controlling = Some(character_entity);
        user_session_data.status = UserStatus::InGame;

        let room = *room_map.0.get(&settings.default_room).unwrap();

        character_created_tx.send(CharacterCreatedEvent(character_entity));

        move_entity_to_room.send(MoveEntityToRoom {
            entity: character_entity,
            room,
        });
    }
}
