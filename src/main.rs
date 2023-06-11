mod network;

use bevy::prelude::*;
use network::NetworkPlugin;

fn main() -> std::io::Result<()> {
  App::new()
    .add_plugins(MinimalPlugins)
    .add_plugin(NetworkPlugin)
    .run();

  Ok(())
}
