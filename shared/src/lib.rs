use bevy::prelude::*;
use prelude::TextEvent;

pub mod character;
pub mod creature;
pub mod display;
pub mod network;
pub mod user;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextEvent>();
    }
}

pub mod prelude {
    pub use crate::character::*;
    pub use crate::creature::*;
    pub use crate::display::*;
    pub use crate::network::*;
    pub use crate::user::*;
}
