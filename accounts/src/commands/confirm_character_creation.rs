use database::prelude::*;
use shared::prelude::*;

use crate::{components::*, events::WelcomeUserEvent};

pub struct ConfirmCharacterCreationCommand;

impl GameCommand for ConfirmCharacterCreationCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<(&SelectedPronouns, &User)>, Res<DbInterface>)> =
            SystemState::new(world);

        let (query, db_interface) = system_state.get(world);

        let (selected_pronouns, user) = match query.get(command.entity) {
            Ok(component) => component,
            Err(_) => return Ok(false),
        };

        if command.keyword == "no" || command.keyword == "n" {
            world.send_event(SendTextToEntityEvent::new(
                command.entity,
                "Alright, let's try again.",
            ));
            world.send_event(SendTextToEntityEvent::new(
                command.entity,
                "What would you like to name your character? It must be between 3 and 15 letters long, and can only contain the letters A-Z.",
            ));
            return Ok(true);
        }

        let settings = db_interface.settings.get_settings().unwrap();

        if command.keyword == "yes" || command.keyword == "y" {
            match db_interface.characters.create_character(
                &selected_pronouns.name,
                selected_pronouns.pronouns,
                settings.default_room,
                user,
            ) {
                Ok(_) => {
                    world.send_event(SendTextToEntityEvent::new(
                        command.entity,
                        "Your character has been created! You can now log in with your new character.",
                    ));

                    world
                        .entity_mut(command.entity)
                        .remove::<SelectedPronouns>()
                        .insert(InLoginMenu {});

                    world.send_event(WelcomeUserEvent(command.entity));
                    return Ok(true);
                }
                Err(_) => {
                    world.send_event(SendTextToEntityEvent::send_generic_error(command.entity));
                    return Ok(true);
                }
            };
        }

        Ok(false)
    }
}
