use std::time::Duration;

use bevy::{
    app::ScheduleRunnerSettings,
    log::{Level, LogPlugin},
    prelude::*,
};
use server::ServerPlugin;

/// Sets up the application to run correctly
pub fn init_app(app: &mut App) -> &mut App {
    app.add_plugin(LogPlugin {
        level: Level::DEBUG,
        filter: "bevy_render=trace,bevy_ecs=trace".to_string(),
    })
    .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .add_plugins(MinimalPlugins)
    .add_plugin(ServerPlugin);
    return app;
}

pub fn run_app(app: &mut App) {
    app.run();
}