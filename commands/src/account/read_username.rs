use bevy::prelude::info;
use shared::user::{User, UserStatus};

use crate::game_command::GameCommand;

pub struct ReadUsername {}

impl GameCommand for ReadUsername {
    fn name(&self) -> String {
        "read_username".to_string()
    }

    fn can_execute(
        &self,
        _command: &Vec<String>,
        acting_entity: &bevy::prelude::Entity,
        world: &bevy::prelude::World,
    ) -> bool {
        let user = world.get::<User>(*acting_entity).unwrap();

        return match user.status {
            UserStatus::NeedUsername => true,
            _ => false,
        };
    }

    fn execute(
        &self,
        command: &Vec<String>,
        acting_entity: &bevy::prelude::Entity,
        world: &mut bevy::prelude::World,
    ) -> Result<(), String> {
        let mut user = world.get_mut::<User>(*acting_entity).unwrap();

        user.username = command[0].clone();

        info!("Username set to: {}", user.username);

        Ok(())
    }
}
