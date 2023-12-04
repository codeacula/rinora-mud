use shared::prelude::*;

use crate::components::*;

pub struct PronounsProvidedCommand;

enum SelectedOption {
    Female,
    Male,
    Both,
}

impl GameCommand for PronounsProvidedCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<Query<&ProvidedCharacterName>> = SystemState::new(world);

        let query = system_state.get(world);

        let character_name = match query.get(command.entity) {
            Ok(name) => name,
            Err(_) => return Ok(false),
        };

        let selected_pronouns: SelectedOption = match command.keyword.as_str() {
            "1" | "she" => SelectedOption::Female,
            "2" | "he" => SelectedOption::Male,
            "3" | "they" | "them" => SelectedOption::Both,
            "she/her" => SelectedOption::Female,
            "he/him" => SelectedOption::Male,
            "they/them" => SelectedOption::Both,
            _ => SelectedOption::Both,
        };

        let mut was_selected = "they/them";

        let selected_pronouns = SelectedPronouns {
            name: character_name.name.clone(),
            pronouns: match selected_pronouns {
                SelectedOption::Both => 2,
                SelectedOption::Female => {
                    was_selected = "she/her";
                    0
                }
                SelectedOption::Male => {
                    was_selected = "he/him";
                    1
                }
            },
        };

        let msg: String = format!("Alright, your character will be referred to as '{}'. This can be changed at any time by looking at CONFIG.\n", was_selected);
        let confirm = format!("Your character's name is '{}', and they'll be addressed as '{}'. If that's correct, send {{{{15}}}}yes{{{{7}}}} to confirm, or {{{{15}}}}no{{{{7}}}} to restart.", character_name.name, was_selected);

        world.send_event(TextEvent::from_str(command.entity, &msg));
        world.send_event(TextEvent::from_str(command.entity, &confirm));
        world.send_event(ShowPromptEvent(command.entity));

        world
            .entity_mut(command.entity)
            .remove::<ProvidedCharacterName>()
            .insert(selected_pronouns);

        return Ok(true);
    }
}
