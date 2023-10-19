use bevy::prelude::*;

#[derive(Event)]
pub struct InvalidCharacterName(pub Entity);
