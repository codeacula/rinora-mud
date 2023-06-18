use std::env;
use std::time::Duration;

use bevy::{
    app::ScheduleRunnerSettings,
    log::{Level, LogPlugin},
    prelude::*,
};
use server::ServerPlugin;

/// Sets up the application to run correctly
pub fn init_app(app: &mut App) -> &mut App {

    let server_host = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
    let server_port = env::var("SERVER_PORT").unwrap_or(String::from("23"));

    app.add_plugin(LogPlugin {
        level: Level::DEBUG,
        filter: "bevy_ecs=trace".to_string(),
    })
    .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .add_plugins(MinimalPlugins)
    .add_plugin(ServerPlugin {
        host: server_host,
        port: server_port,
    });
    return app;
}

pub fn run_app(app: &mut App) {
    app.run();
}
