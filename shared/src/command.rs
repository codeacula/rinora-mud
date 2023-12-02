use bevy::{prelude::*, utils::Uuid};

#[derive(Debug, Event)]
pub struct UserProvidedCommandEvent {
    pub id: Uuid,
    pub entity: Entity,
    pub command: String,
}

#[derive(Debug, Event)]
pub struct UserProvidedGmcpEvent {
    pub id: Uuid,
    pub entity: Entity,
    pub command: String,
    pub data: String,
}

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

impl UserCommand {
    pub fn new(raw_command: String) -> Self {
        let mut parts = raw_command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let keyword = parts.remove(0);
        UserCommand {
            full_command: raw_command.trim().to_string(),
            entity: Entity::PLACEHOLDER,
            keyword,
            parts,
            raw_command,
        }
    }
}

pub trait GameCommand: Sync + Send {
    /// Executes the command. Returns false if the command wasn't applied, and true if it were
    /// Application means that the command was valid, the user was in a valid state to run the command, and its effects
    /// were applied.
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String>;
}

pub struct GameCommandEvent(Box<dyn GameCommand>);

impl<T: GameCommand + 'static> From<T> for GameCommandEvent {
    fn from(value: T) -> Self {
        let command: Box<dyn GameCommand> = Box::new(value);
        GameCommandEvent(command)
    }
}

/// GameCommands are all the commands a user can run
#[derive(Resource)]
pub struct GameCommands(pub Vec<Box<dyn GameCommand>>);
