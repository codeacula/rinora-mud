use bevy::prelude::App;
use main::{init_app, run_app};

pub fn main() {
    let mut app = App::new();
    init_app(&mut app);
    run_app(&mut app);
}
