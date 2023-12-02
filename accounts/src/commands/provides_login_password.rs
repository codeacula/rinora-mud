use database::prelude::*;
use shared::prelude::*;

use crate::{components::*, events::*};

pub struct ProvidesLoginPassword;

fn send_not_found(world: &mut World, entity: Entity) -> Result<bool, String> {
    world.send_event(AccountNotFoundEvent(entity));
    world
        .entity_mut(entity)
        .remove::<LoggingIn>()
        .insert(NeedsUsername {});

    return Ok(true);
}

impl GameCommand for ProvidesLoginPassword {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&LoggingIn>, Res<DbInterface>)> =
            SystemState::new(world);

        let (query, db_interface) = system_state.get(world);

        let logging_in = match query.get(command.entity) {
            Ok(logging_in) => logging_in,
            Err(_) => {
                return Ok(false);
            }
        };

        let user = match db_interface
            .users
            .find_with_credentials(&logging_in.username, &command.full_command)
        {
            Ok(user_option) => match user_option {
                Some(user) => user,
                None => {
                    return send_not_found(world, command.entity);
                }
            },
            Err(err) => {
                error!("Error checking if user exists: {}", err);
                return send_not_found(world, command.entity);
            }
        };

        world.entity_mut(command.entity).insert(user);

        return Ok(true);
    }
}
