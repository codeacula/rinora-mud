use shared::prelude::*;

pub struct ConfirmAccountPasswordCommand {}

impl GameCommand for ConfirmAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut query = world.query::<&UserSessionData>();

        let Ok(user_sesh) = query.get(world, command.entity) else {
            return Ok(false);
        };

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
    use super::*;

    #[test]
    fn doesnt_run_if_no_user_sesh() {
        let mut app = build_test_app();
        let command = build_user_command(String::from("password"), Entity::PLACEHOLDER);

        let result = ConfirmAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn doesnt_run_if_no_password() {
        let mut app = build_test_app();

        let mut entity_builder = EntityBuilder::new();
        let user_sesh = UserSessionData::new();
        entity_builder.set_session_data(user_sesh);

        let command = build_user_command(String::from(""), entity_builder.build(&mut app.world));

        let result = ConfirmAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(false));

        let evs = app.world.resource::<Events<GenericErrorEvent>>();
        assert_eq!(evs.len(), 1);
    }

    #[test]
    fn works_but_procuses_password_does_not_match_if_passwords_dont_match() {
        let mut app = build_test_app();
        app.add_event::<ConfirmPasswordDoesNotMatchEvent>();

        let mut entity_builder = EntityBuilder::new();
        let mut user_sesh = UserSessionData::new();
        user_sesh.pwd = Some(String::from("password"));
        entity_builder.set_session_data(user_sesh);

        let command = build_user_command(String::from(""), entity_builder.build(&mut app.world));

        let result = ConfirmAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(true));

        let evs = app
            .world
            .resource::<Events<ConfirmPasswordDoesNotMatchEvent>>();
        assert_eq!(evs.len(), 1);
    }

    #[test]
    fn works() {
        let mut app = build_test_app();
        app.add_event::<UserConfirmedPasswordEvent>();

        let mut entity_builder = EntityBuilder::new();
        let mut user_sesh = UserSessionData::new();
        user_sesh.pwd = Some(String::from("password"));
        entity_builder.set_session_data(user_sesh);

        let command = build_user_command(
            String::from("password"),
            entity_builder.build(&mut app.world),
        );

        let result = ConfirmAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(true));

        let evs = app.world.resource::<Events<UserConfirmedPasswordEvent>>();
        assert_eq!(evs.len(), 1);
    }
}
