use bevy::prelude::*;
use command::PossibleCommands;
use prelude::TextEvent;

pub mod character;
pub mod command;
pub mod creature;
pub mod display;
pub mod network;
pub mod user;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<TextEvent>();

        // Commands
        app.insert_resource(PossibleCommands(Vec::new()));
    }
}

pub mod prelude {
    pub use crate::character::*;
    pub use crate::command::*;
    pub use crate::creature::*;
    pub use crate::display::*;
    pub use crate::network::*;
    pub use crate::user::*;
}
