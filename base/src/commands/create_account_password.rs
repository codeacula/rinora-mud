use shared::prelude::*;

pub struct CreateAccountPasswordCommand {}

impl GameCommand for CreateAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let password = command.full_command.clone();

        if password.len() < 3 {
            world.send_event(PasswordNotLongEnoughEvent(command.entity));
            return Ok(true);
        }

        world.send_event(UserProvidedPasswordEvent {
            user_entity: command.entity,
            password,
        });

        Ok(true)
    }
}
