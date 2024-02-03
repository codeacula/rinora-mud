use shared::prelude::*;

use crate::components::*;

pub struct NewCharacterNameProvidedCommand;

impl GameCommand for NewCharacterNameProvidedCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<Query<&InCharacterCreation>> = SystemState::new(world);

        let query = system_state.get(world);

        if query.get(command.entity).is_err() {
            return Ok(false);
        }

        if !command.parts.is_empty() {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Your character name can't have spaces in it.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        if command.keyword.len() < 3 {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Your character name needs to be at least three characters long.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        if command.keyword.len() > 15 {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Your character name can't be more than 15 characters long.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        if !command.keyword.chars().all(|c| c.is_alphabetic()) {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Your character name can only contain letters.",
            ));
            world.send_event(ShowPromptEvent(command.entity));
            return Ok(true);
        }

        // Clean up the name some
        let name = clean_name(&command.keyword);
        world
            .entity_mut(command.entity)
            .remove::<InCharacterCreation>()
            .insert(ProvidedCharacterName { name: name.clone() });

        world.send_event(TextEvent::from_str(
            command.entity,
            &format!(
                "Alright. Your character will be known as <<15>>{}<<7>>.\n\nWhat are your characters pronous?\n  1. She/her\n  2. He/him\n  3. They/them\n\n(Pronouns have no mechanical effect on the game.)",
                name
            ),
        ));
        world.send_event(ShowPromptEvent(command.entity));

        Ok(true)
    }
}
