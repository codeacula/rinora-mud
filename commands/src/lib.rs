mod account;
mod game_command;

use bevy::prelude::*;
use shared::prelude::*;

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
    query: Query<&User>,
    mut ev_incoming_commands: EventReader<InputReceivedEvent>,
    mut ev_outgoing_account_events: EventWriter<AccountEvent>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    for command in ev_incoming_commands.iter() {
        let user = query.get(command.entity).unwrap();
        let cleaned_command = clean_incoming_command(command.input.as_bytes().to_vec());

        if user.status == UserStatus::InGame {
            if cleaned_command[0] == "say" {
                let mut message = String::new();
                for word in cleaned_command.iter().skip(1) {
                    message.push_str(word);
                    message.push(' ');
                }
                outgoing_queue.send_str(user.connection, &format!("You say: {}\n", message));
                return;
            }

            if cleaned_command[0] == "butts" {
                outgoing_queue.send_str(user.connection, "if(asstrack.score == 42069)\n");
                return;
            }

            outgoing_queue.send_str(user.connection, "Invalid command!\n");
            return;
        }

        ev_outgoing_account_events.send(AccountEvent {
            entity: command.entity,
            input: cleaned_command,
        });
    }
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AccountEvent>()
            .add_systems(First, process_incoming_commands);
    }
}
