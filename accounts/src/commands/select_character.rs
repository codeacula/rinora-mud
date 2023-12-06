use database::prelude::*;
use shared::prelude::*;

use crate::components::*;

pub struct SelectCharacterCommand;

impl GameCommand for SelectCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&User, With<InLoginMenu>>, Res<DbInterface>)> =
            SystemState::new(world);

        let (query, db_interface) = system_state.get(world);

        let user_info = match query.get(command.entity) {
            Ok(user_info) => user_info,
            Err(_) => return Ok(false),
        };

        let character_name = command.keyword.clone();

        if !db_interface
            .characters
            .does_user_own_character(&character_name, user_info.id)
        {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Sorry, we couldn't locate that character. Please try again.",
            ));

            world.send_event(ShowPromptEvent(command.entity));

            return Ok(true);
        };

        let get_character_result = db_interface
            .characters
            .get_character_by_name(&character_name);

        let username = user_info.username.clone();

        let character = match get_character_result {
            Ok(Some(character)) => character,
            Ok(None) => {
                world.send_event(TextEvent::send_generic_error(command.entity));
                world.send_event(ShowPromptEvent(command.entity));
                error!(
                    "User {} has character in DB but couldn't pull it for some reason.",
                    username
                );
                return Ok(true);
            }
            Err(e) => {
                world.send_event(TextEvent::send_generic_error(command.entity));
                world.send_event(ShowPromptEvent(command.entity));
                error!(
                    "User {} has character in DB but couldn't pull it for some reason. Error: {}",
                    username, e
                );
                return Ok(true);
            }
        };

        let mut character_entity = world.spawn(character);
        character_entity.insert(IsControlledBy(command.entity));

        return Ok(true);
    }
}
