use bevy::prelude::*;
use command::PossibleCommands;
use prelude::*;

pub mod account;
pub mod character;
pub mod command;
pub mod creature;
pub mod display;
pub mod helpers;
pub mod network;
pub mod room;
pub mod user;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Account
        app.add_event::<AccountEvent>()
            .add_event::<UserLoggedIn>()
            .add_event::<UserProvidedUsername>()
            .add_event::<UserProvidedPassword>()
            .add_event::<UserSelectedLoginOption>()
            .add_event::<UserProvidedCharacterToDelete>()
            .add_event::<UserCreatedPassword>()
            .add_event::<UserConfirmedPassword>()
            .add_event::<UserProvidedCharacterName>()
            .add_event::<UserConfirmedDeleteCharacter>();

        // Characters
        app.add_event::<DeleteCharacterEvent>();

        // Commands
        app.insert_resource(PossibleCommands(Vec::new()));

        // Events
        app.add_event::<TextEvent>();
    }
}

pub mod prelude {
    pub use crate::account::*;
    pub use crate::character::*;
    pub use crate::command::*;
    pub use crate::creature::*;
    pub use crate::display::*;
    pub use crate::helpers::string::*;
    pub use crate::helpers::*;
    pub use crate::network::*;
    pub use crate::room::*;
    pub use crate::user::*;
}
