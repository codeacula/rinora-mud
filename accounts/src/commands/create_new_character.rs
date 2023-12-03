use database::prelude::*;
use shared::prelude::*;

use crate::components::*;

pub struct CreateNewCharacterCommand;

impl GameCommand for CreateNewCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&LoggingIn>, Res<DbInterface>)> =
            SystemState::new(world);

        let (query, db_interface) = system_state.get(world);

        return Ok(false);
    }
}
