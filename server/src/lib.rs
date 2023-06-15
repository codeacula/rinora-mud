pub struct ServerPlugin;

use bevy::prelude::*;

fn start_listening() {

}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_listening);
    }
}