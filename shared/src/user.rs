use bevy::{prelude::Component, utils::Uuid};

#[derive(Component)]
pub struct Login;

#[derive(Debug, Component)]
pub struct User {
    pub connection: Uuid,
}
