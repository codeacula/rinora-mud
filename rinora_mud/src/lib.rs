use account::AccountPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use commands::CommandsPlugin;
use server::NetworkServerPlugin;

pub fn start_game() {
    let mut app = App::new();

    let commands: Vec<Box<dyn GameCommand>> = Vec::new();

    app.add_plugins(LogPlugin {
        level: Level::DEBUG,
        filter: "debug,rinora_mud=debug".into(),
    })
    .insert_non_send_resource(commands)
    .add_plugins((
        MinimalPlugins,
        AccountPlugin,
        NetworkServerPlugin,
        CommandsPlugin,
    ))
    .run()
}
