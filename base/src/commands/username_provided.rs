use database::prelude::*;
use shared::prelude::*;

pub struct UsernameProvidedCommand {}

impl GameCommand for UsernameProvidedCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(
            Res<DbInterface>,
            Query<&mut UserSessionData>,
            EventWriter<TextEvent>,
        )> = SystemState::new(world);
        let (db_repo, mut query, mut text_event_tx) = system_state.get_mut(world);

        let username = &command.keyword;

        let Ok(user_sesh) = query.get(command.entity) else {
            return Ok(false);
        };

        if !is_alphabetic(username) {
            world.send_event(UsernameInvalid(command.entity));
            return Ok(true);
        }

        let user_exists = match db_repo.users.does_user_exist(username) {
            Ok(exists) => exists,
            Err(e) => {
                text_event_tx.send(TextEvent::send_generic_error(command.entity));
                return Err(format!("Error while checking if user exists: {:?}", e));
            }
        };

        let mut user_sesh = query.get_mut(command.entity).unwrap();
        user_sesh.username = username.to_string();

        if user_exists {
            user_sesh.status = UserStatus::NeedPassword;
            text_event_tx.send(TextEvent::from_str(
                command.entity,
                "User account found. Please provide your password.",
            ));
        } else {
            user_sesh.status = UserStatus::CreatePassword;
            text_event_tx.send(TextEvent::from_str(
                command.entity,
                "Welcome, new user! What should your password be?",
            ));
        }
        Ok(true)
    }
}
