mod account;
mod game_command;

use bevy::{ecs::system::SystemState, prelude::*};
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

pub fn process_incoming_commands(world: &mut World) {
    let mut event_system_state: SystemState<EventReader<InputReceivedEvent>> =
        SystemState::new(world);
    let mut events = event_system_state.get_mut(world);

    let game_commands = world
        .get_non_send_resource::<Vec<Box<dyn GameCommand>>>()
        .unwrap();

    for event in events.iter() {
        let cleaned_command = clean_incoming_command(event.input.clone().as_bytes().to_vec());

        for boxed_command in game_commands.iter() {
            let command = boxed_command.as_ref();

            if command.can_execute(&cleaned_command, &event.entity, world) {
                command.execute(&cleaned_command, &event.entity, world);
            }
        }
    }
}

fn add_commands(commands: NonSend<Vec<Box<dyn GameCommand>>>) {
    commands.push(Box::new(account::read_username::ReadUsername {}))
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_commands)
            .add_systems(First, process_incoming_commands);
    }
}
