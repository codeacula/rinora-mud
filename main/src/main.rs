use bevy::prelude::*;

fn main() -> std::io::Result<()> {
  App::new()
    .add_plugins(MinimalPlugins)
    .add_plugin(NetworkPlugin)
    .run();

  Ok(())
}
