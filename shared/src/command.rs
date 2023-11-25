use bevy::{prelude::*, utils::HashMap};

use crate::prelude::UserStatus;

/// UserCommand contains the information from the text command that was sent in. This gets converted into the actual
/// command that will run
#[derive(Debug, Clone)]
pub struct UserCommand {
    /// The command the user sent, cleaned up
    pub full_command: String,

    /// The Entity responsible for sending the command
    pub entity: Entity,

    /// The main keyword of the command. Should be equivalent to `parts[0]`, with some exceptions for commands like
    /// `'Hello` which should expand to SAY HELLO. The command interpreter will handle those specially
    pub keyword: String,

    /// Each part of a command. So if someone sent "bash camel hard" the value should be ["bash", "camel", "hard"]
    pub parts: Vec<String>,

    /// The command exactly as provided, including the new newline
    pub raw_command: String,
}

pub trait GameCommand: Sync + Send {
    /// Executes the command. Returns false if the command applied, and true if it did
    /// Application doesn't just mean matches the right pattern. There are plenty of commands that may only work when
    /// a user is there, but then we want to fall back if they aren't.
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String>;
}

pub struct GameCommandEvent(Box<dyn GameCommand>);

impl<T: GameCommand + 'static> From<T> for GameCommandEvent {
    fn from(value: T) -> Self {
        let command: Box<dyn GameCommand> = Box::new(value);
        GameCommandEvent(command)
    }
}

/// GameCommands are only ran when the user is logged into a character. They're sorted by the user status so we don't
/// have to check the status each time
#[derive(Resource)]
pub struct GameCommands(pub HashMap<UserStatus, Vec<Box<dyn GameCommand>>>);
