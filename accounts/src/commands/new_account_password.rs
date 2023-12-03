use database::prelude::*;
use shared::prelude::*;

use crate::{components::*, events::WelcomeUserEvent};

pub struct NewAccountPasswordCommand;

impl GameCommand for NewAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&mut CreatingAccount>, Res<DbInterface>)> =
            SystemState::new(world);

        let (mut query, db_interface) = system_state.get_mut(world);

        let mut creating_account = match query.get_mut(command.entity) {
            Ok(logging_in) => logging_in,
            Err(_) => {
                debug!("User {:?} not creating an account", command.entity);
                return Ok(false);
            }
        };

        if command.full_command.len() < 3 {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Your password needs to be at least three characters long. Please try again.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        if creating_account.password.is_none() {
            creating_account.password = Some(command.full_command.clone());
            world.send_event(TextEvent::from_str(
                command.entity,
                "Please confirm your password.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        let password = creating_account.password.clone().unwrap();
        if password == command.full_command {
            let user = match db_interface
                .users
                .create_user(&creating_account.username, &password)
            {
                Ok(user) => user,
                Err(err) => {
                    error!("Error creating user: {}", err);
                    world.send_event(TextEvent::send_generic_error(command.entity));
                    world.send_event(ShowPromptEvent(command.entity));
                    return Ok(true);
                }
            };

            world
                .entity_mut(command.entity)
                .remove::<CreatingAccount>()
                .insert(user);

            world.send_event(WelcomeUserEvent(command.entity));
            return Ok(true);
        }

        creating_account.password = None;
        world.send_event(TextEvent::from_str(
            command.entity,
            "Your passwords didn't match. Please try again.",
        ));
        world.send_event(ShowPromptEvent(command.entity));
        return Ok(true);
    }
}
