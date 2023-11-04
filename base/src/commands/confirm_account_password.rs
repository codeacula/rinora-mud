use shared::prelude::*;

pub struct ConfirmAccountPasswordCommand {}

impl GameCommand for ConfirmAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut query = world.query::<&UserSessionData>();

        let Ok(user_sesh) = query.get(world, command.entity) else {
            return Ok(false);
        };

        info!("User Session Data {user_sesh:?}");

        let original_password = match &user_sesh.pwd {
            Some(val) => val,
            None => {
                error!("Expected user to have a stored password, but didn't!");
                world.send_event(GenericErrorEvent(command.entity));
                return Ok(false);
            }
        };

        let confirmation_password = &command.full_command;

        if original_password != confirmation_password {
            world.send_event(ConfirmPasswordDoesNotMatchEvent(command.entity));
            return Ok(true);
        }

        world.send_event(UserConfirmedPasswordEvent(command.entity));
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use shared::prelude::*;

    use crate::commands::prelude::ConfirmAccountPasswordCommand;

    #[test]
    fn doesnt_run_if_no_user_sesh() {
        let app = build_test_app();
        let command = build_user_command(String::from("password"));
        let mut world = app.world;

        let result = ConfirmAccountPasswordCommand {}.run(&command, &mut world);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn doesnt_run_if_no_password() {
        let app = build_test_app();
        let mut world = app.world;
        build_entity(&mut world);

        let command = build_user_command(String::from(""));

        let result = ConfirmAccountPasswordCommand {}.run(&command, &mut world);
        assert_eq!(result, Ok(false));

        let evs = world.resource::<Events<GenericErrorEvent>>();
        assert_eq!(evs.len(), 1);
    }

    #[test]
    fn works_but_procuses_password_does_not_match_if_passwords_dont_match() {}

    #[test]
    fn works() {}
}
