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
    query: Query<(Entity, &UserSessionData)>,
    possible_commands: Res<PossibleCommands>,
    mut ev_incoming_commands: EventReader<InputReceivedEvent>,
    mut ev_outgoing_account_events: EventWriter<AccountEvent>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
) {
    for command in ev_incoming_commands.iter() {
        let (entity, user_sesh) = query.get(command.entity).unwrap();
        let cleaned_command = clean_incoming_command(command.input.as_bytes().to_vec());

        if user_sesh.status == UserStatus::InGame {
            if cleaned_command[0] == "say" {
                let mut message = String::new();
                for word in cleaned_command.iter().skip(1) {
                    message.push_str(word);
                    message.push(' ');
                }
                ev_outgoing_text_events
                    .send(TextEvent::new(entity, &format!("You say: {}\n", message)));
                return;
            }

            if cleaned_command[0] == "butts" {
                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "{{11:0}}if{{15:0}}({{208:0}}asstrack.score {{15:0}}== {{141:0}}42069{{15:0}})",
                ));
                return;
            }

            if possible_commands.0.contains(&cleaned_command[0]) {
                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "{{15:0}}You've provided a valid command that isn't implemented yet.",
                ));
                return;
            }

            ev_outgoing_text_events.send(TextEvent::new(entity, &String::from("Invalid command!")));
            return;
        }

        ev_outgoing_account_events.send(AccountEvent {
            entity: command.entity,
            input: cleaned_command,
            raw_command: command.input.clone(),
        });
    }
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, process_incoming_commands);
    }
}
