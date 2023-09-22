use bevy::prelude::*;

#[derive(Resource)]
pub struct PossibleCommands(pub Vec<String>);

#[derive(Clone)]
pub struct SentCommand {
    // The command the user sent, cleaned up
    pub full_command: String,

    /// The Entity responsible for sending the command
    pub entity: Entity,

    /// The main keyword of the command. Should be equivalent to parts[0], with some exceptions for commands like
    /// `'Hello` which should expand to SAY HELLO. The command interpreter will handle those specially
    pub keyword: String,

    /// Each part of a command. So if someone sent "bash camel hard" the value should be ["bash", "camel", "hard"]
    pub parts: Vec<String>,

    /// The command exactly as provided, including the new newline
    pub raw_command: String,
}
