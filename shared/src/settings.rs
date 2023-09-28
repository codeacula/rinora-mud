use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
  pub support_email: String,
  pub default_room: i32,
}
