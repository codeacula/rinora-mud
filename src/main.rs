mod network;

use bevy::prelude::*;
use network::NetworkManagerPlugin;

fn main() {
  App::new()
    .add_plugins(MinimalPlugins)
    .add_plugin(NetworkManagerPlugin)
    .run();
}
