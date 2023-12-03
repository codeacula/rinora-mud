use database::prelude::*;
use shared::prelude::*;

use crate::{components::*, events::WelcomeUserEvent};

pub struct ProvidesLoginPasswordCommand;

fn send_not_found(world: &mut World, entity: Entity) -> Result<bool, String> {
    world.send_event(TextEvent::from_str(
        entity,
        "I'm sorry, it looks like that's the wrong password. Let's start over. What's your username?",
    ));
    world.send_event(ShowPromptEvent(entity));

    world
        .entity_mut(entity)
        .remove::<LoggingIn>()
        .insert(NeedsUsername {});

    return Ok(true);
}

impl GameCommand for ProvidesLoginPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&LoggingIn>, Res<DbInterface>)> =
            SystemState::new(world);

        let (query, db_interface) = system_state.get(world);

        let logging_in = match query.get(command.entity) {
            Ok(logging_in) => logging_in,
            Err(_) => {
                debug!("User {:?} not logging in", command.entity);
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
                    debug!("User {:?} not found", command.entity);
                    return send_not_found(world, command.entity);
                }
            },
            Err(err) => {
                error!("Error checking if user exists: {}", err);
                return send_not_found(world, command.entity);
            }
        };

        info!("User {} logged in", logging_in.username);
        world
            .entity_mut(command.entity)
            .remove::<LoggingIn>()
            .insert(user);

        world.send_event(WelcomeUserEvent(command.entity));

        return Ok(true);
    }
}
