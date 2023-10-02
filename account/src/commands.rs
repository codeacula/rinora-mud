use bevy::prelude::*;
use shared::prelude::*;

pub struct UsernameProvided {}

impl GameCommand for UsernameProvided {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status == UserStatus::NeedUsername {
            return true;
        }

        return false;
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let test_binding = world.get_mut::<UserSessionData>(command.entity);

        let Some(mut user_session) = test_binding else {
            return Err("Couldn't locate the user session data to modify".to_string());
        };

        user_session.username = command.full_command.clone();
        user_session.status = UserStatus::NeedPassword;
        Ok(())
    }
}
