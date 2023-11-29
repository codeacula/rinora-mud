use base::*;
use shared::prelude::*;

pub fn start_game() {
    let mut app = App::new();
    app.add_plugins(BaseRinoraPlugin).run()
}
 