use bevy::prelude::*;
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

/// Takes an entity and the command they sent and converts it into a SentCommand
fn create_sent_command(entity: Entity, command: Vec<u8>) -> UserCommand {
    let command_string = String::from_utf8(command).unwrap();
    let cleaned_string = command_string.replace(|c: char| !c.is_ascii(), "");

    let parts: Vec<String> = cleaned_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    UserCommand {
        full_command: cleaned_string,
        entity,
        keyword: parse_keyword(&parts[0]),
        parts,
        raw_command: command_string,
    }
}

fn get_user_input_events(world: &mut World) -> Vec<InputReceivedEvent> {
    world
        .resource_mut::<Events<InputReceivedEvent>>()
        .drain()
        .collect::<Vec<InputReceivedEvent>>()
}

pub fn process_incoming_commands(world: &mut World) {
    let user_input_events = get_user_input_events(world);

    world.resource_scope(|world, game_commands: Mut<GameCommands>| {
        for user_input in user_input_events {
            let entity = world.entity(user_input.entity).clone().id();
            let sent_command = create_sent_command(entity, user_input.input.as_bytes().to_vec());

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
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        let command_list = GameCommands(Vec::new());
        app.insert_resource(command_list)
            .add_systems(First, process_incoming_commands);
    }
}
