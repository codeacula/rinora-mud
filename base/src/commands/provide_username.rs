use shared::prelude::*;

pub struct ProvideUsernameCommand {}

impl GameCommand for ProvideUsernameCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let username = &command.keyword;

        error!("Das fake error");

        if !is_valid_username(username) {
            world.send_event(UsernameInvalidEvent(command.entity));
            return Ok(true);
        }

        world.send_event(UsernameProvidedEvent {
            user_entity: command.entity,
            username: username.to_string(),
        });

        Ok(true)
    }
}
