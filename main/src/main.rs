use bevy::{app::ScheduleRunnerSettings, prelude::*, utils::Duration};
use server::*;

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(ServerPlugin)
        .run();
}
