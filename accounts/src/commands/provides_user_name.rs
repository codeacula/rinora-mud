use database::prelude::*;
use shared::prelude::*;

use crate::{components::*, events::*};

pub struct ProvidesUserNameCommand;

impl GameCommand for ProvidesUserNameCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(
            Query<Entity, (With<UserSessionData>, With<NeedsUsername>)>,
            Res<DbInterface>,
        )> = SystemState::new(world);

        let (query, db_interface) = system_state.get(world);

        if !query.contains(command.entity) {
            return Ok(false);
        }

        if command.parts.len() != 0 {
            world.send_event(InvalidUsernameFormatEvent(command.entity));
            return Ok(true);
        }

        let user_exists = match db_interface.users.does_user_exist(&command.keyword) {
            Ok(user_exists) => user_exists,
            Err(err) => {
                error!("Error checking if user exists: {}", err);
                world.send_event(TextEvent::send_generic_error(command.entity));
                return Ok(true);
            }
        };

        let mut entity = world.entity_mut(command.entity);
        entity.remove::<NeedsUsername>();

        if !user_exists {
            entity.insert(CreatingAccount {
                username: command.keyword.clone(),
                password: None,
            });
            world.send_event(CreatingNewAccountEvent(command.entity));
            return Ok(true);
        }

        entity.insert(LoggingIn {
            username: command.keyword.clone(),
        });
        world.send_event(LoggingInEvent(command.entity));
        Ok(true)
    }
}
