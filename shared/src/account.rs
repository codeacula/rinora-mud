use bevy::{ecs::system::Command, prelude::*};

use crate::prelude::{UserSessionData, UserStatus};

pub struct TransitionUserToState {
    pub entity: Entity,
    pub state: UserStatus,
}

impl Command for TransitionUserToState {
    fn apply(self, world: &mut World) {
        let Some(mut found_entity) = world.get_entity_mut(self.entity) else {
            error!("Unable to transition user state: Entity not found");
            return;
        };

        let Some(mut user) = found_entity.get_mut::<UserSessionData>() else {
            error!("Unable to transition user state: User now found");
            return;
        };

        user.status = self.state;
    }
}

#[derive(Event)]
pub struct AccountEvent {
    pub entity: Entity,
    pub input: Vec<String>,
    pub raw_command: String,
}

#[derive(Event)]
pub struct UserLoggedIn {
    pub entity: Entity,
    pub uuid: String,
}
