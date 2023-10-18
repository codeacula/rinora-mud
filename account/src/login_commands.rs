use bevy::{ecs::system::SystemState, prelude::*};
use database::prelude::*;
use shared::prelude::*;

pub struct UserConfirmedPassword {}

impl GameCommand for UserConfirmedPassword {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status == UserStatus::ConfirmPassword {
            return true;
        }

        false
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let mut system_state: SystemState<(
            Res<DbInterface>,
            Query<&mut UserSessionData>,
            EventWriter<TextEvent>,
            EventWriter<UserLoggedIn>,
            Commands,
        )> = SystemState::new(world);
        let (db_repo, mut query, mut text_event_tx, mut user_logged_in_tx, mut commands) =
            system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        if user_sesh.pwd.is_none() {
            error!("User got into ConfirmPassword state without having a password set in session!");
            text_event_tx.send(TextEvent::send_generic_error(command.entity));
            return Ok(());
        }

        let original_password = user_sesh.pwd.as_ref().unwrap();
        let confirmation_password = &command.full_command;

        if original_password != confirmation_password {
            text_event_tx.send(TextEvent::from_str(
                command.entity,
                "Your passwords don't match, let's try again. What password do you want?",
            ));

            user_sesh.status = UserStatus::CreatePassword;
            return Ok(());
        }

        let new_user = match db_repo
            .users
            .create_user(&user_sesh.username, confirmation_password)
        {
            Ok(uuid) => uuid,
            Err(err) => {
                error!("Unable to create user: {err}");
                text_event_tx.send(TextEvent::send_generic_error(command.entity));
                return Ok(());
            }
        };

        commands.entity(command.entity).insert(User {
            autologin: new_user.autologin,
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
        user_logged_in_tx.send(UserLoggedIn {
            entity: command.entity,
            id: new_user.id,
        });
        Ok(())
    }
}

pub struct UsernameProvided {}

impl GameCommand for UsernameProvided {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status == UserStatus::NeedUsername {
            return true;
        }

        false
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let mut system_state: SystemState<(
            Res<DbInterface>,
            Query<&mut UserSessionData>,
            EventWriter<TextEvent>,
        )> = SystemState::new(world);
        let (db_repo, mut query, mut text_event_tx) = system_state.get_mut(world);

        let username = &command.keyword;

        if !is_alphabetic(username) {
            text_event_tx.send(TextEvent::new(
                command.entity,
                &"Only alphabetic (a-z) characters are allowed.".to_string(),
            ));
            return Ok(());
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
        Ok(())
    }
}

pub struct PasswordCreated {}

impl GameCommand for PasswordCreated {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status == UserStatus::CreatePassword {
            return true;
        }

        false
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let mut system_state: SystemState<(Query<&mut UserSessionData>, EventWriter<TextEvent>)> =
            SystemState::new(world);
        let (mut query, mut text_event_tx) = system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        let password = command.full_command.clone();

        user_sesh.pwd = Some(password);
        user_sesh.status = UserStatus::ConfirmPassword;

        text_event_tx.send(TextEvent::from_str(
            command.entity,
            "Excellent. Now, provide your password again for confirmation.",
        ));
        Ok(())
    }
}

pub struct PasswordProvided {}

impl GameCommand for PasswordProvided {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status == UserStatus::NeedPassword {
            return true;
        }

        false
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let mut system_state: SystemState<(
            Res<DbInterface>,
            Query<&mut UserSessionData>,
            EventWriter<TextEvent>,
            EventWriter<UserLoggedIn>,
            Commands,
        )> = SystemState::new(world);
        let (db_repo, mut query, mut text_event_tx, mut user_logged_in_tx, mut commands) =
            system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        let provided_password = command.full_command.clone();

        let user_option = match db_repo
            .users
            .find_with_credentials(&user_sesh.username, &provided_password)
        {
            Ok(user) => user,
            Err(e) => {
                error!("Error while logging in user: {:?}", e);
                text_event_tx.send(TextEvent::send_generic_error(command.entity));
                return Ok(());
            }
        };

        if user_option.is_none() {
            text_event_tx.send(TextEvent::from_str(
                command.entity,
                "Looks like there's a problem with the password. Let's try again. What's your username?",
            ));
            user_sesh.username.clear();
            user_sesh.status = UserStatus::NeedUsername;
            return Ok(());
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

        user_logged_in_tx.send(UserLoggedIn {
            entity: command.entity,
            id: user.id,
        });

        system_state.apply(world);
        Ok(())
    }
}
