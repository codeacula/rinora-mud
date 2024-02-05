use base::BaseRinoraPlugin;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(BaseRinoraPlugin).run()
}
