use std::time::Duration;

use account::AccountPlugin;
use bevy::{
    app::ScheduleRunnerSettings,
    log::{Level, LogPlugin},
    prelude::*,
};
use server::GameServer;

pub fn start_game() {
    let mut app = App::new();

    app.add_plugin(LogPlugin {
        level: Level::DEBUG,
        filter: "bevy_ecs=trace".to_string(),
    })
    .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .add_plugins(MinimalPlugins)
    .add_plugin(AccountPlugin)
    .add_plugin(GameServer)
    .run()
}
