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
fn create_sent_command(entity: Entity, command: Vec<u8>) -> SentCommand {
    let command_string = String::from_utf8(command).unwrap();
    let cleaned_string = command_string.replace(|c: char| !c.is_ascii(), "");

    let parts: Vec<String> = cleaned_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    SentCommand {
        full_command: cleaned_string,
        entity,
        keyword: parse_keyword(&parts[0]),
        parts,
        raw_command: command_string,
    }
}

pub fn process_incoming_commands(
    query: Query<(Entity, &UserSessionData)>,
    possible_commands: Res<PossibleCommands>,
    mut ev_incoming_commands: EventReader<InputReceivedEvent>,
    mut ev_outgoing_account_events: EventWriter<AccountEvent>,
    mut commands: Commands,
) {
    for command in ev_incoming_commands.iter() {
        let (entity, user_sesh) = query.get(command.entity).unwrap();
        let sent_command = create_sent_command(entity, command.input.as_bytes().to_vec());

        if user_sesh.status == UserStatus::InGame {
            if sent_command.keyword == "say" {
                let mut message = String::new();
                for word in sent_command.parts.iter().skip(1) {
                    message.push_str(word);
                    message.push(' ');
                }

                commands.add(SendText::new(entity, &format!("You say: {}\n", message)));
                return;
            }

            if sent_command.keyword == "butts" {
                commands.add(SendText::new(
                    entity,
                    "{{11:0}}if{{15:0}}({{208:0}}asstrack.score {{15:0}}== {{141:0}}42069{{15:0}})",
                ));
                return;
            }

            if possible_commands.0.contains(&sent_command.keyword) {
                commands.add(SendText::new(
                    entity,
                    "{{15:0}}You've provided a valid command that isn't implemented yet.",
                ));
                return;
            }

            commands.add(SendText::new(entity, "Invalid command!"));
            return;
        }

        ev_outgoing_account_events.send(AccountEvent {
            entity: command.entity,
            command: sent_command,
        });
    }
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, process_incoming_commands);
    }
}
