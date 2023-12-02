use database::prelude::*;
use shared::prelude::*;

use crate::{components::*, events::*};

pub struct ProvidesUserNameCommand;

impl GameCommand for ProvidesUserNameCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(
            Query<(&mut UserSessionData, &NeedsUsername)>,
            Res<DbInterface>,
        )> = SystemState::new(world);

        let (mut query, db_interface) = system_state.get_mut(world);

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
                return Ok(true);
            }
        };

        let (mut user_sesh, _) = query.get_mut(command.entity).unwrap();
        user_sesh.username = Some(command.keyword.clone());
        let mut entity = world.entity_mut(command.entity);
        entity.remove::<NeedsUsername>();

        if !user_exists {
            entity.insert(NeedsToProvideNewPassword {});
            world.send_event(CreatingNewAccountEvent(command.entity));
            return Ok(true);
        }

        entity.insert(NeedsAccountPassword {});
        world.send_event(ConfirmingPasswordEvent(command.entity));
        Ok(true)
    }
}
