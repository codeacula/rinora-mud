use account::AccountPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use commands::CommandsPlugin;
use database::DatabasePlugin;
use server::NetworkServerPlugin;

pub fn start_game() {
    let mut app = App::new();

    app.add_plugins(LogPlugin {
        level: Level::DEBUG,
        filter: "debug,rinora_mud=debug".into(),
    })
    .add_plugins((
        MinimalPlugins,
        DatabasePlugin,
        AccountPlugin,
        NetworkServerPlugin,
        CommandsPlugin,
    ))
    .run()
}
