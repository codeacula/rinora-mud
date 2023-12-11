use shared::prelude::*;

use crate::components::*;

pub struct SelectedCreateNewCharacterCommand;

impl GameCommand for SelectedCreateNewCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<Query<&InLoginMenu>> = SystemState::new(world);

        let query = system_state.get(world);

        if query.get(command.entity).is_err() {
            return Ok(false);
        }

        if command.keyword != "1" {
            return Ok(false);
        }

        world
            .entity_mut(command.entity)
            .remove::<InLoginMenu>()
            .insert(InCharacterCreation {});

        world.send_event(TextEvent::from_str(
            command.entity,
            "What would you like to name your character? It must be between 3 and 15 letters long, and can only contain the letters A-Z.",
        ));
        world.send_event(ShowPromptEvent(command.entity));

        return Ok(true);
    }
}
