use bevy::prelude::*;

#[derive(Resource)]
pub struct PossibleCommands(pub Vec<String>);

/// UserCommand contains the information from the text command that was sent in. This gets converted into the actual
/// command that will run
#[derive(Debug, Clone)]
pub struct UserCommand {
    /// The command the user sent, cleaned up
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

pub trait GameCommand: Sync + Send {
    /// Given a command, determines if it can run
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool;

    /// Execute the command against the World
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String>;
}

pub struct GameCommandEvent(Box<dyn GameCommand>);

impl<T: GameCommand + 'static> From<T> for GameCommandEvent {
    fn from(value: T) -> Self {
        let command: Box<dyn GameCommand> = Box::new(value);
        GameCommandEvent(command)
    }
}

/// AccountCommands are only ran when the user isn't logged into a character
#[derive(Resource)]
pub struct AccountCommands(pub Vec<Box<dyn GameCommand>>);

/// GameCommands are only ran when the user is logged into a character
#[derive(Resource)]
pub struct GameCommands(pub Vec<Box<dyn GameCommand>>);
