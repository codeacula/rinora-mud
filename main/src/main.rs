use bevy::{
  app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
  prelude::*,
  utils::Duration,
};
use server::*;

fn main() {
  let desired_loop = ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(1.0 / 60.0));
  App::new()
    .add_plugins(MinimalPlugins.set(desired_loop))
    .add_plugin(ServerPlugin)
    .run();
}
