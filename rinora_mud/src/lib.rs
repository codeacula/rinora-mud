use account::AccountPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use commands::CommandsPlugin;
use database::prelude::*;
use helper::HelperPlugin;
use server::NetworkServerPlugin;
use shared::prelude::*;

mod commands;
mod display;
mod world;

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
        HelperPlugin,
    ))
    .add_systems(
        Update,
        world::add_character_to_room.in_set(GameOrderSet::Game),
    )
    .add_systems(
        Update,
        (
            display::display_character_entering_room,
            display::display_character_logged_into_room,
            display::display_room_to_user,
        )
            .in_set(GameOrderSet::Output),
    )
    .run()
}
