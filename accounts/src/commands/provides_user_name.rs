use database::prelude::*;
use shared::prelude::*;

use crate::components::*;

pub struct ProvidesUserNameCommand;

impl GameCommand for ProvidesUserNameCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        debug!("Running ProvidesUserNameCommand");
        let mut system_state: SystemState<(
            Query<Entity, (With<UserSessionData>, With<NeedsUsername>)>,
            Res<DbInterface>,
        )> = SystemState::new(world);

        let (query, db_interface) = system_state.get(world);

        if !query.contains(command.entity) {
            return Ok(false);
        }

        if !command.parts.is_empty() {
            debug!("Entity {:?} provided too many words.", command.entity);
            world.send_event(TextEvent::from_str(
                command.entity,
                "Account names can't have spaces in them.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        if !is_valid_username(&command.keyword) {
            debug!(
                "Entity {:?} provided a non-alphabetic username",
                command.entity
            );
            world.send_event(TextEvent::from_str(
                command.entity,
                "That's in invalid username. Usernames must start with a letter, be between 3 and 15 characters long, and must only have letters, numbers, or underscores. Please try again.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
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
            debug!("Tagging {:?} with CreatingAccount", command.entity);
            entity.insert(CreatingAccount {
                username: command.keyword.clone(),
                password: None,
            });

            world.send_event(TextEvent::from_str(
                command.entity ,
                "It looks like you're new here. What would you like your password to be? It needs to be at least three characters long."
            ));

            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        debug!("Tagging {:?} with LoggingIn", command.entity);
        entity.insert(LoggingIn {
            username: command.keyword.clone(),
        });

        world.send_event(TextEvent::from_str(
            command.entity,
            "Welcome back! What's your password?",
        ));
        world.send_event(ShowPromptEvent(command.entity));
        Ok(true)
    }
}
