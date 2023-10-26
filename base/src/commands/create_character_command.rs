use shared::prelude::*;

pub struct CreateCharacterCommand {}

impl GameCommand for CreateCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return Ok(false);
        };

        if user_session.status != UserStatus::LoggedIn {
            return Ok(false);
        }

        Ok(true)
    }
}
