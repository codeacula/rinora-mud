use shared::prelude::*;

pub struct CreateCharacterCommand {}

impl GameCommand for CreateCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        if command.keyword != "1" {
            return Ok(false);
        }

        world.send_event(CreateCharacterSelectedEvent(command.entity));
        Ok(true)
    }
}
