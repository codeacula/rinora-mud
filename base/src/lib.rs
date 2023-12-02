use accounts::AccountPlugin;
use bevy::log::{Level, LogPlugin};
use database::prelude::*;
use helper::*;
use networking::NetworkPlugin;
use shared::prelude::*;

mod enums;
mod events;
mod gmcp;
mod helpers;
mod stream_processor;

pub struct BaseRinoraPlugin;

impl Plugin for BaseRinoraPlugin {
    fn build(&self, app: &mut App) {
        app
            // System Plugins
            .add_plugins(LogPlugin {
                level: Level::DEBUG,
                filter: "debug,rinora_mud=debug".into(),
            })
            // Plugins
            .add_plugins((
                MinimalPlugins,
                SharedPlugin,
                DatabasePlugin,
                HelperPlugin,
                NetworkPlugin,
                AccountPlugin,
            ));
    }
}
