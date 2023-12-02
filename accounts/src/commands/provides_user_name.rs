use shared::prelude::*;

pub struct ProvidesUserNameCommand;

impl GameCommand for ProvidesUserNameCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        info!("We made it in here: {:?}", command.entity);
        Ok(true)
    }
}
