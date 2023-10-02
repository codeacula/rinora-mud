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

pub fn process_incoming_commands_old(
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

fn get_user_input_events(world: &mut World) -> Vec<InputReceivedEvent> {
    world
    .resource_mut::<Events<InputReceivedEvent>>()
    .drain()
    .collect::<Vec<InputReceivedEvent>>() 
}

pub fn process_incoming_commands(world: &mut World) {
    let user_input_events = get_user_input_events(world);
    let command_list = world.resource::<GameCommands>();

    let mut found_commands: Vec<(UserCommand, &Box<dyn GameCommand>)> = Vec::new();

    for user_input in user_input_events {
        let entity = world.entity(user_input.entity).clone().id();
        let sent_command = create_sent_command(entity, user_input.input.as_bytes().to_vec());

        for game_command in command_list.0.iter() {
            if game_command.can_execute(&sent_command, world) {
                found_commands.push((sent_command, game_command));
                break;
            }
        }
    }

    let parts = found_commands.clone();

    for (user_command, game_command) in parts {
        game_command.run(&user_command, world);
    }
}

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        let command_list = GameCommands(Vec::new());
        app.insert_resource(command_list)
            .add_systems(First, process_incoming_commands)
            .add_systems(First, process_incoming_commands_old);
    }
}
