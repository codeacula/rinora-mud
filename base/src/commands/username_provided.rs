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
