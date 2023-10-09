use account::AccountPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use commands::CommandsPlugin;
use database::DatabasePlugin;
use display::*;
use server::NetworkServerPlugin;
use shared::SharedPlugin;

mod commands;
mod display;

pub fn start_game() {
    let mut app = App::new();

    app.add_plugins(LogPlugin {
        level: Level::DEBUG,
        filter: "debug,rinora_mud=debug".into(),
    })
    .add_plugins((
        MinimalPlugins,
        SharedPlugin,
        CommandsPlugin,
        DatabasePlugin,
        AccountPlugin,
        NetworkServerPlugin,
    ))
    .add_systems(Update, display_room_to_user)
    .run()
}
