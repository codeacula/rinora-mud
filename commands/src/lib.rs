mod account;
mod game_command;

use bevy::prelude::*;
use game_command::GameCommand;
use shared::network::InputReceivedEvent;

pub struct CommandsPlugin;

fn clean_incoming_command(command: Vec<u8>) -> Vec<String> {
    let command_string = String::from_utf8(command).unwrap();
    let cleaned_string = command_string.replace(|c: char| !c.is_ascii(), "");

    cleaned_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn process_incoming_commands(
    commands: NonSend<Vec<Box<dyn GameCommand>>>,
    mut ev_incoming_commands: EventReader<InputReceivedEvent>,
    mut world: World,
) {
    for ev in ev_incoming_commands.iter() {
        let command_parts = clean_incoming_command(ev.input.clone().into_bytes());

        if command_parts.len() == 0 {
            continue;
        }

        for command in commands.iter() {
            if command.can_execute(command_parts.clone(), &ev.entity, &world) {
                if let Err(e) = command.execute(command_parts.clone(), &ev.entity, &mut world) {
                    error!("Error executing command: {}", e);
                }
            }
        }
    }
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        let command: Vec<Box<dyn GameCommand>> =
            vec![Box::new(account::read_username::ReadUsername {})];

        app.insert_non_send_resource(command)
            .add_systems(First, process_incoming_commands);
    }
}
