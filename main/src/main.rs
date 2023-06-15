use bevy::prelude::*;
use server::*;

fn main() {
  App::new()
    .add_plugins(MinimalPlugins)
    .add_plugin(ServerPlugin)
    .run();
}
