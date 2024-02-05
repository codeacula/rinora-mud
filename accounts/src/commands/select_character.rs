use database::prelude::*;
use shared::prelude::*;

use crate::components::*;

pub struct SelectCharacterCommand;

impl GameCommand for SelectCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        // Go ahead and set the character_name we'll use in the function
        let character_name = command.keyword.clone();

        // System State
        let mut system_state: SystemState<(Query<&User, With<InLoginMenu>>, Res<DbInterface>)> =
            SystemState::new(world);
        let (query, db_interface) = system_state.get_mut(world);

        // User info
        let user_info = match query.get(command.entity) {
            Ok(user_info) => user_info,
            Err(_) => return Ok(false),
        };

        let username = user_info.username.clone();

        // Character validation
        if !db_interface
            .characters
            .does_user_own_character(&character_name, &user_info.id)
        {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Sorry, we couldn't locate that character. Please try again.",
            ));

            world.send_event(ShowPromptEvent(command.entity));

            return Ok(true);
        };

        let mut character = match db_interface
            .characters
            .get_character_by_name(&character_name)
        {
            Ok(Some(character)) => character,
            Ok(None) => {
                world.send_event(TextEvent::send_generic_error(command.entity));
                world.send_event(ShowPromptEvent(command.entity));
                error!("User {username} has character in DB but couldn't pull it for some reason.",);
                return Ok(true);
            }
            Err(e) => {
                world.send_event(TextEvent::send_generic_error(command.entity));
                world.send_event(ShowPromptEvent(command.entity));
                error!(
                    "User {username} has character in DB but couldn't pull it for some reason. Error: {e}"
                );
                return Ok(true);
            }
        };

        // Make sure we put the room entity in the location
        let location_id = character.location.location_id;
        let room_map = world.get_resource::<RoomMap>().unwrap();
        let room = match room_map.0.get(&location_id) {
            Some(room) => *room,
            None => {
                world.send_event(TextEvent::send_generic_error(command.entity));
                world.send_event(ShowPromptEvent(command.entity));
                error!("Couldn't locate room {location_id} in room map!",);
                return Ok(true);
            }
        };
        character.location.entity = room;

        let mut character_entity = world.spawn(character);
        character_entity.insert(IsControlledBy(command.entity));
        let character_entity_id = character_entity.id();

        world.send_event(CharacterLoggedInEvent(character_entity_id));

        world
            .entity_mut(command.entity)
            .remove::<InLoginMenu>()
            .insert(InGame {});

        Ok(true)
    }
}
