mod account;
mod game_command;

use bevy::prelude::*;
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

pub fn process_incoming_commands(mut ev_incoming_commands: EventReader<InputReceivedEvent>) {
    for ev in ev_incoming_commands.iter() {
        let command_parts = clean_incoming_command(ev.input.clone().into_bytes());

        if command_parts.len() == 0 {
            continue;
        }

        let command = command_parts[0].to_lowercase();

        match command.as_str() {
            "quit" => {
                debug!("Quitting");
                std::process::exit(0);
            }
            "help" => {
                debug!("Help");
            }
            _ => {
                debug!("Unknown command: {}", command);
            }
        }
    }
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_incoming_commands);
    }
}
