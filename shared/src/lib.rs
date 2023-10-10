use bevy::prelude::*;
use command::PossibleCommands;
use prelude::*;

pub mod being;
pub mod collections;
pub mod command;
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
  Network,
  Command,
  Account,
  Game,
  Cleanup,
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Configure system sets
        app.configure_set(First, GameOrderSet::Network.before(GameOrderSet::Command));
        app.configure_set(First, GameOrderSet::Command.before(GameOrderSet::Account));
        app.configure_set(First, GameOrderSet::Account.before(GameOrderSet::Game));
        app.configure_set(First, GameOrderSet::Game.before(GameOrderSet::Cleanup));

        app.configure_set(PreUpdate, GameOrderSet::Network.before(GameOrderSet::Command));
        app.configure_set(PreUpdate, GameOrderSet::Command.before(GameOrderSet::Account));
        app.configure_set(PreUpdate, GameOrderSet::Account.before(GameOrderSet::Game));
        app.configure_set(PreUpdate, GameOrderSet::Game.before(GameOrderSet::Cleanup));

        app.configure_set(Update, GameOrderSet::Network.before(GameOrderSet::Command));
        app.configure_set(Update, GameOrderSet::Command.before(GameOrderSet::Account));
        app.configure_set(Update, GameOrderSet::Account.before(GameOrderSet::Game));
        app.configure_set(Update, GameOrderSet::Game.before(GameOrderSet::Cleanup));

        app.configure_set(Last, GameOrderSet::Network.before(GameOrderSet::Command));
        app.configure_set(Last, GameOrderSet::Command.before(GameOrderSet::Account));
        app.configure_set(Last, GameOrderSet::Account.before(GameOrderSet::Game));
        app.configure_set(Last, GameOrderSet::Game.before(GameOrderSet::Cleanup));

        // Account
        app.add_event::<UserLoggedIn>();

        // Commands
        app.insert_resource(PossibleCommands(Vec::new()));

        // Entities
        app.add_event::<EntityEnteredRoom>()
            .add_event::<EntityEnteredWorld>();

        // Events
        app.add_event::<TextEvent>();
    }
}

pub mod prelude {
    pub use crate::being::*;
    pub use crate::collections::*;
    pub use crate::command::*;
    pub use crate::display::*;
    pub use crate::helpers::string::*;
    pub use crate::helpers::*;
    pub use crate::network::*;
    pub use crate::settings::*;
    pub use crate::status::*;
    pub use crate::user::*;
    pub use crate::world::*;

    pub use crate::GameOrderSet;
    pub use crate::SharedPlugin;
}
