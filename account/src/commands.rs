use bevy::prelude::*;
use database::prelude::*;
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
        let db_repo = world.remove_resource::<DbInterface>().unwrap();

        let username = &command.keyword;

        if !is_alphabetic(username) {
            world.send_event(TextEvent::new(command.entity, &"Only alphabetic (a-z) characters are allowed.".to_string()));
            return Ok(());
        }

        let user_exists = match db_repo.users.does_user_exist(username) {
            Ok(exists) => exists,
            Err(e) => {
                world.send_event(TextEvent::send_generic_error(command.entity));
                return Err(format!("Error while checking if user exists: {:?}", e));
            }
        };

        world.insert_resource(db_repo);

        let mut user_sesh = world.get_mut::<UserSessionData>(command.entity).unwrap();

        user_sesh.username = username.clone();

        if user_exists {
            user_sesh.status = UserStatus::NeedPassword;
            world.send_event(TextEvent::from_str(command.entity, "User account found. Please provide your password."));
        } else {
            user_sesh.status = UserStatus::CreatePassword;
            world.send_event(TextEvent::from_str(command.entity, "Welcome, new user! What should your password be?"));
        }
        Ok(())
    }
}
