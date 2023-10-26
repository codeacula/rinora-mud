use shared::prelude::*;

pub struct ConfirmAccountPasswordCommand {}

impl GameCommand for ConfirmAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut query = world.query::<&UserSessionData>();

        let Ok(user_sesh) = query.get(world, command.entity) else {
            return Ok(false);
        };

        if user_sesh.pwd.is_none() {
            error!("Expect the user to have a session, but doesn't.");
            world.send_event(GenericErrorEvent(command.entity));
            return Ok(false);
        }

        let original_password = user_sesh.pwd.as_ref().unwrap();
        let confirmation_password = &command.full_command;

        if original_password != confirmation_password {
            world.send_event(ConfirmPasswordDoesntMatchEvent(command.entity));
            return Ok(true);
        }

        world.send_event(UserConfirmedPasswordEvent(command.entity));
        Ok(true)
    }
}
