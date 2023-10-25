use database::prelude::*;
use shared::prelude::*;

pub struct UserConfirmedPasswordCommand {}

impl GameCommand for UserConfirmedPasswordCommand {
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

        if user_sesh.status != UserStatus::ConfirmPassword {
            return Ok(false);
        }

        if user_sesh.pwd.is_none() {
            error!("User got into ConfirmPassword state without having a password set in session!");
            world.send_event(GenericErrorEvent(command.entity));
            return Ok(false);
        }

        let original_password = user_sesh.pwd.as_ref().unwrap();
        let confirmation_password = &command.full_command;

        if original_password != confirmation_password {
            text_event_tx.send(TextEvent::from_str(
                command.entity,
                "Your passwords don't match, let's try again. What password do you want?",
            ));

            user_sesh.status = UserStatus::CreatePassword;
            return Ok(true);
        }

        let new_user = match db_repo
            .users
            .create_user(&user_sesh.username, confirmation_password)
        {
            Ok(uuid) => uuid,
            Err(err) => {
                error!("Unable to create user: {err}");
                text_event_tx.send(TextEvent::send_generic_error(command.entity));
                return Ok(true);
            }
        };

        commands.entity(command.entity).insert(User {
            id: new_user.id,
            username: user_sesh.username.clone(),
            administrator: new_user.administrator,
            current_character: None,
        });

        text_event_tx.send(TextEvent::from_str(
            command.entity,
            "Passwords match, account created! You are now logged in.\n\n",
        ));

        user_sesh.status = UserStatus::LoggedIn;
        user_logged_in_tx.send(UserLoggedInEvent {
            entity: command.entity,
            id: new_user.id,
        });
        Ok(true)
    }
}
