use shared::prelude::*;

use crate::components::*;

pub struct PronounsProvidedCommand;

impl GameCommand for PronounsProvidedCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<Query<&ProvidedCharacterName>> = SystemState::new(world);

        let query = system_state.get(world);

        if query.get(command.entity).is_err() {
            return Ok(false);
        }

        return Ok(true);
    }
}
