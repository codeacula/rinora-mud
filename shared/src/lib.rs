use bevy::prelude::*;
use prelude::*;

pub mod account;
pub mod being;
pub mod collections;
pub mod command;
pub mod content;
pub mod display;
pub mod helpers;
pub mod network;
pub mod settings;
pub mod status;
pub mod user;
pub mod world;

pub struct SharedPlugin;

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet)]
pub enum GameOrderSet {
    Network, // Ran first, so that all connections are handled and commands are sent to the command handlers
    Command, // The command step is meant to handle processing commands from the network layer
    Account, // The account section is a separate state from normal, and requires running first
    Pre, // The is for systems we want to run before the game, such as adding users to the game world
    Game, // This is where all the game systems are updated
    Post, // This is for systems that want to run after the world has processed but before cleanup
    Cleanup, // Cleanup is meant to handle stuff like storing removed characters and stuff
    Debug, // Debug is specifically so its output is sent before normal output
    Output, // Flush all output here
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Configure system sets
        app.configure_set(First, GameOrderSet::Network.before(GameOrderSet::Command));
        app.configure_set(First, GameOrderSet::Command.before(GameOrderSet::Account));
        app.configure_set(First, GameOrderSet::Account.before(GameOrderSet::Pre));
        app.configure_set(First, GameOrderSet::Pre.before(GameOrderSet::Game));
        app.configure_set(First, GameOrderSet::Game.before(GameOrderSet::Post));
        app.configure_set(First, GameOrderSet::Post.before(GameOrderSet::Cleanup));
        app.configure_set(First, GameOrderSet::Cleanup.before(GameOrderSet::Debug));
        app.configure_set(First, GameOrderSet::Debug.before(GameOrderSet::Output));

        app.configure_set(
            PreUpdate,
            GameOrderSet::Network.before(GameOrderSet::Command),
        );
        app.configure_set(
            PreUpdate,
            GameOrderSet::Command.before(GameOrderSet::Account),
        );
        app.configure_set(PreUpdate, GameOrderSet::Account.before(GameOrderSet::Pre));
        app.configure_set(PreUpdate, GameOrderSet::Pre.before(GameOrderSet::Game));
        app.configure_set(PreUpdate, GameOrderSet::Game.before(GameOrderSet::Post));
        app.configure_set(PreUpdate, GameOrderSet::Post.before(GameOrderSet::Cleanup));
        app.configure_set(PreUpdate, GameOrderSet::Cleanup.before(GameOrderSet::Debug));
        app.configure_set(PreUpdate, GameOrderSet::Debug.before(GameOrderSet::Output));

        app.configure_set(Update, GameOrderSet::Network.before(GameOrderSet::Command));
        app.configure_set(Update, GameOrderSet::Command.before(GameOrderSet::Account));
        app.configure_set(Update, GameOrderSet::Account.before(GameOrderSet::Pre));
        app.configure_set(Update, GameOrderSet::Pre.before(GameOrderSet::Game));
        app.configure_set(Update, GameOrderSet::Game.before(GameOrderSet::Post));
        app.configure_set(Update, GameOrderSet::Post.before(GameOrderSet::Cleanup));
        app.configure_set(Update, GameOrderSet::Cleanup.before(GameOrderSet::Debug));
        app.configure_set(Update, GameOrderSet::Debug.before(GameOrderSet::Output));

        app.configure_set(Last, GameOrderSet::Network.before(GameOrderSet::Command));
        app.configure_set(Last, GameOrderSet::Command.before(GameOrderSet::Account));
        app.configure_set(Last, GameOrderSet::Account.before(GameOrderSet::Pre));
        app.configure_set(Last, GameOrderSet::Pre.before(GameOrderSet::Game));
        app.configure_set(Last, GameOrderSet::Game.before(GameOrderSet::Post));
        app.configure_set(Last, GameOrderSet::Post.before(GameOrderSet::Cleanup));
        app.configure_set(Last, GameOrderSet::Cleanup.before(GameOrderSet::Debug));
        app.configure_set(Last, GameOrderSet::Debug.before(GameOrderSet::Output));

        // Account
        app.add_event::<UserLoggedInEvent>()
            .add_event::<CharacterCreatedEvent>()
            .add_event::<CharacterExistsEvent>()
            .add_event::<CharacterNameInvalidEvent>()
            .add_event::<CharacterNotFoundEvent>();

        // Commands
        app.add_event::<GenericErrorEvent>();

        // Entities
        app.add_event::<EntityEnteredRoomEvent>()
            .add_event::<EntityEnteredWorldEvent>()
            .add_event::<EntityLeftRoomEvent>()
            .add_event::<EntityLeftWorldEvent>();

        // Events
        app.add_event::<ShowLoginScreenEvent>()
            .add_event::<ShowPromptEvent>()
            .add_event::<TextEvent>();
    }
}

pub mod prelude {
    pub use crate::account::*;
    pub use crate::being::*;
    pub use crate::collections::*;
    pub use crate::command::*;
    pub use crate::content::*;
    pub use crate::display::*;
    pub use crate::helpers::string::*;
    pub use crate::helpers::test::*;
    pub use crate::helpers::*;
    pub use crate::network::*;
    pub use crate::settings::*;
    pub use crate::status::*;
    pub use crate::user::*;
    pub use crate::world::*;

    pub use crate::GameOrderSet;
    pub use crate::SharedPlugin;

    pub use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap, utils::Uuid};
}
