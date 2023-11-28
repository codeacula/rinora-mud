use crate::events::*;
use shared::prelude::*;

/// Takes an InputReceivedEvent and converts it into a SentCommand
pub fn create_sent_command(event: &InputReceivedEvent) -> UserCommand {
    let command_string = event.input.clone();
    let cleaned_string = command_string.trim().replace(|c: char| !c.is_ascii(), "");

    let parts: Vec<String> = cleaned_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    UserCommand {
        full_command: cleaned_string,
        entity: event.entity,
        keyword: parse_keyword(&parts[0]),
        parts,
        raw_command: command_string,
    }
}

/// Makes a copy of the InputReceivedEvents and returns them
fn get_user_input_events(world: &mut World) -> Vec<InputReceivedEvent> {
    world
        .resource_mut::<Events<InputReceivedEvent>>()
        .drain()
        .collect::<Vec<InputReceivedEvent>>()
}

/// Given a word, determines if its a special case for a keyword. Otherwise, just return the word. This will allow us to
/// have commands like 'butts convert to "say butts"
pub fn parse_keyword(command: &str) -> String {
    if command.starts_with('\'') {
        return "say".to_string();
    } else if command.starts_with(';') {
        return "emote".to_string();
    }

    command.trim().to_string()
}

pub fn process_incoming_commands(world: &mut World) {
    let user_input_events = get_user_input_events(world);

    // Go ahead and take these out now so we don't have to deal with borrower issues7
    let game_commands = world.remove_resource::<GameCommands>().unwrap();

    for user_input in user_input_events {
        let sent_command = create_sent_command(&user_input);

        // Unwrap is safe here because they can't get here without UserSessionData
        let user_sesh = world.get::<UserSessionData>(sent_command.entity).unwrap();

        let Some(commands_to_check) = game_commands.0.get(&user_sesh.status) else {
            continue;
        };

        let mut did_send_command = false;

        for game_command in commands_to_check {
            let run_result = match game_command.run(&sent_command, world) {
                Ok(val) => val,
                Err(e) => {
                    error!("There was an error attempting to run command: {e:?}");
                    world.send_event(GenericErrorEvent(sent_command.entity));
                    return;
                }
            };

            if run_result {
                did_send_command = true;
                break;
            }
        }

        if !did_send_command {
            world.send_event(InvalidCommandEvent(sent_command.entity));
        }
    }

    // We need to put the resources back when done
    world.insert_resource(game_commands);
}
