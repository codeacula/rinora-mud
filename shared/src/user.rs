use bevy::{prelude::Component, utils::Uuid};

pub enum UserStatus {
    NeedsUsername,
    NeedsPassword,
    Authenticated,
    ConfirmCreate,
    CreatingPassword,
    ConfirmingPassword,
}

#[derive(Component)]
pub struct User {
    pub connection: Uuid,
    pub status: UserStatus,
}
