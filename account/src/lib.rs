use bevy::prelude::*;

pub struct AccountPlugin;

fn handle_new_connections() {}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_new_connections);
    }
}
