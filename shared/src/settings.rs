use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    pub support_email: String,
    pub default_room: i32,
}
