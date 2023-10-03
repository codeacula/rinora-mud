use bevy::prelude::*;
use command::PossibleCommands;
use prelude::*;

pub mod character;
pub mod command;
pub mod creature;
pub mod display;
pub mod helpers;
pub mod network;
pub mod settings;
pub mod user;
pub mod world;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Account
        app.add_event::<AccountEvent>().add_event::<UserLoggedIn>();

        // Characters
        app.add_event::<DeleteCharacterEvent>();

        // Commands
        app.insert_resource(PossibleCommands(Vec::new()));

        // Events
        app.add_event::<TextEvent>();
    }
}

pub mod prelude {
    pub use crate::character::*;
    pub use crate::command::*;
    pub use crate::creature::*;
    pub use crate::display::*;
    pub use crate::helpers::string::*;
    pub use crate::helpers::*;
    pub use crate::network::*;
    pub use crate::settings::*;
    pub use crate::user::*;
    pub use crate::world::*;
}
