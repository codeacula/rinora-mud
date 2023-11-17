use database::prelude::DbInterface;
use shared::prelude::*;

pub fn log_character_into_game(
    mut character_selected_rx: EventReader<LogCharacterInEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
    room_map: Res<RoomMap>,
    mut query: Query<&mut UserSessionData>,
    mut character_logged_in_tx: EventWriter<CharacterLoggedInEvent>,
) {
    for ev in character_selected_rx.read() {
        // Fetch the CharacterBundle from the database. There's also a bunch of error checking.
        let mut character = match db_repo.characters.get_character_by_name(&ev.character_name) {
            Ok(character) => match character {
                Some(character) => character,

                None => {
                    error!("Character not found: {}", ev.character_name);
                    continue;
                }
            },

            Err(e) => {
                error!("Error getting character: {e:?}");
                continue;
            }
        };

        // Update the user's location with the right entity
        let location_id = character.location.location_id;
        let room = room_map
            .0
            .get(&location_id)
            .expect("Unable to locate room in map: {location_id}");
        character.location.entity = * room;

        let mut character_ent = commands.spawn(character);

        character_ent.insert((IsControlledBy(ev.user_entity), EntityIsLoggingIn));

        let mut user_sesh = query.get_mut(ev.user_entity).unwrap();
        user_sesh.status = UserStatus::InGame;
        user_sesh.controlling_entity = Some(character_ent.id());

        character_logged_in_tx.send(CharacterLoggedInEvent(character_ent.id()));
    }
}
