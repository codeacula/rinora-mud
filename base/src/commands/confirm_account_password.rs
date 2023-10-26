use shared::prelude::*;

pub struct ConfirmAccountPasswordCommand {}

impl GameCommand for ConfirmAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&mut UserSessionData>,)> = SystemState::new(world);
        let mut query = system_state.get_mut(world);

        let Ok(user_sesh) = query.get(command.entity) else {
            return Ok(false);
        };

        if user_sesh.pwd.is_none() {
            error!("User got into ConfirmPassword state without having a password set in session!");
            world.send_event(GenericErrorEvent(command.entity));
            return Ok(false);
        }

        let original_password = user_sesh.pwd.as_ref().unwrap();
        let confirmation_password = &command.full_command;

        if original_password != confirmation_password {
            world.send_event(ConfirmPasswordDoesntMatch(command.entity));
            return Ok(true);
        }

        world.send_event(UserConfirmedPassword(command.entity));
        Ok(true)
    }
}
