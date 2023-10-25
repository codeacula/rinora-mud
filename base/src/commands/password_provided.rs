use database::prelude::*;
use shared::prelude::*;

pub struct PasswordProvided {}

impl GameCommand for PasswordProvided {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(
            Res<DbInterface>,
            Query<&mut UserSessionData>,
            EventWriter<TextEvent>,
            EventWriter<UserLoggedInEvent>,
            Commands,
        )> = SystemState::new(world);
        let (db_repo, mut query, mut text_event_tx, mut user_logged_in_tx, mut commands) =
            system_state.get_mut(world);

        let Ok(mut user_sesh) = query.get_mut(command.entity) else {
            return Ok(false);
        };

        if user_sesh.status != UserStatus::NeedPassword {
            return Ok(false);
        }

        let provided_password = command.full_command.clone();

        let user_option = match db_repo
            .users
            .find_with_credentials(&user_sesh.username, &provided_password)
        {
            Ok(user) => user,
            Err(e) => {
                error!("Error while logging in user: {:?}", e);
                text_event_tx.send(TextEvent::send_generic_error(command.entity));
                return Ok(true);
            }
        };

        if user_option.is_none() {
            text_event_tx.send(TextEvent::from_str(
                command.entity,
                "Looks like there's a problem with the password. Let's try again. What's your username?",
            ));
            user_sesh.username.clear();
            user_sesh.status = UserStatus::NeedUsername;
            return Ok(true);
        }

        text_event_tx.send(TextEvent::from_str(
            command.entity,
            "Thank you! Welcome back!\n\n",
        ));

        let user = user_option.unwrap();
        user_sesh.status = UserStatus::LoggedIn;

        if user.administrator {
            commands.entity(command.entity).insert(IsAdmin);
        }

        user_logged_in_tx.send(UserLoggedInEvent {
            entity: command.entity,
            id: user.id,
        });

        system_state.apply(world);
        Ok(true)
    }
}
