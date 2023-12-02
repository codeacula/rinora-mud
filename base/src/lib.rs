use accounts::AccountPlugin;
use bevy::log::{Level, LogPlugin};
use database::prelude::*;
use helper::*;
use networking::NetworkPlugin;
use shared::prelude::*;
use systems::run_user_commands::*;

mod enums;
mod events;
mod helpers;
mod systems;

pub struct BaseRinoraPlugin;

impl Plugin for BaseRinoraPlugin {
    fn build(&self, app: &mut App) {
        let game_commands: GameCommands = GameCommands(Vec::new());

        app
            // System Plugins
            .add_plugins(LogPlugin {
                level: Level::DEBUG,
                filter: "debug,rinora_mud=debug".into(),
            })
            .insert_resource(game_commands)
            // Plugins
            .add_plugins((
                MinimalPlugins,
                SharedPlugin,
                DatabasePlugin,
                HelperPlugin,
                NetworkPlugin,
                AccountPlugin,
            ))
            .add_systems(First, (run_user_commands).in_set(GameOrderSet::Command));
    }
}
