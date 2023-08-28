use bevy::prelude::Component;

use crate::creature::Health;

#[derive(Component)]
pub struct Character {
    pub health: Health,
}
