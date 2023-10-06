use bevy::{ecs::system::SystemState, prelude::*};
use shared::prelude::*;

pub struct CommandsPlugin;

/// Given a word, determines if its a special case for a keyword. Otherwise, just return the word. This will allow us to
/// have commands like 'butts convert to "say butts"
fn parse_keyword(command: &str) -> String {
    if command.starts_with('\'') {
        return "say".to_string();
    } else if command.starts_with(';') {
        return "emote".to_string();
    }

    command.to_string()
}

/// Takes an InputReceivedEvent and converts it into a SentCommand
fn create_sent_command(event: &InputReceivedEvent) -> UserCommand {
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
fn get_user_input_events(world: &mut World) -> Vec<UserCommand> {
    world
        .resource_mut::<Events<InputReceivedEvent>>()
        .drain()
        .map(|cmd| create_sent_command(&cmd))
        .collect::<Vec<UserCommand>>()
}

pub fn process_incoming_commands(world: &mut World) {
    let user_input_events = get_user_input_events(world);
    let mut system_state: SystemState<(
        Res<AccountCommands>,
        Res<GameCommands>,
        Query<&UserSessionData>,
    )> = SystemState::new(world);
    let (account_commands, game_commands, query) = system_state.get(world);

    for sent_command in user_input_events {
        let user_sesh: &UserSessionData = query.get(sent_command.entity).unwrap();

        if user_sesh.status == UserStatus::InGame {
            for game_command in game_commands.0.iter() {
                if game_command.can_execute(&sent_command, world) {
                    if let Err(e) = game_command.run(&sent_command, world) {
                        error!("There was an error attempting to run command: {}", e);
                    }
                    break;
                }
            }
        } else {
            for game_command in account_commands.0.iter() {
                if game_command.can_execute(&sent_command, world) {
                    if let Err(e) = game_command.run(&sent_command, world) {
                        error!("There was an error attempting to run command: {}", e);
                    }
                    break;
                }
            }
        }
    }
}

/*pub fn process_incoming_commands(world: &mut World) {
    let user_input_events = get_user_input_events(world);

    world.resource_scope(|world, game_commands: Mut<GameCommands>| {
        for user_input in user_input_events {
            let sent_command = create_sent_command(&user_input);

            for game_command in game_commands.0.iter() {
                if game_command.can_execute(&sent_command, world) {
                    if let Err(e) = game_command.run(&sent_command, world) {
                        error!("There was an error attempting to run command: {}", e);
                    }

                    break;
                }
            }
        }
    });
}*/

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        let command_list = GameCommands(Vec::new());
        app.insert_resource(command_list)
            .add_systems(First, process_incoming_commands);
    }
}
