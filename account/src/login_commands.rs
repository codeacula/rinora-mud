use bevy::prelude::*;
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

        return false;
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let (entity, user_sesh) = world
            .query::<(Entity, &UserSessionData)>()
            .get_mut(world, command.entity)
            .unwrap();

        if user_sesh.pwd.is_none() {
            error!("User got into ConfirmPassword state without having a password set in session!");
            world.send_event(TextEvent::send_generic_error(entity));
            return Ok(());
        }

        let original_password = user_sesh.pwd.as_ref().unwrap();
        let confirmation_password = &command.full_command;

        if original_password != confirmation_password {
            world.send_event(TextEvent::from_str(
                entity,
                "Your passwords don't match, let's try again. What password do you want?",
            ));

            user_sesh.status = UserStatus::CreatePassword;
            return Ok(());
        }

        let db_repo = world.resource::<DbInterface>();

        let new_user = match db_repo
            .users
            .create_user(&user_sesh.username, confirmation_password)
        {
            Ok(uuid) => uuid,
            Err(e) => {
                error!("Unable to create user: {:?}", e);
                world.send_event(TextEvent::send_generic_error(entity));
                return Ok(());
            }
        };

        world.entity_mut(entity).insert(User {
            autologin: new_user.autologin,
            id: new_user.id,
            username: user_sesh.username.clone(),
            administrator: new_user.administrator,
        });

        world.send_event(TextEvent::from_str(
            entity,
            "Passwords match, account created! You are now logged in.\n\n",
        ));

        user_sesh.status = UserStatus::LoggedIn;
        world.send_event(UserLoggedIn {
            entity,
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

        return false;
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let username = &command.keyword;

        if !is_alphabetic(username) {
            world.send_event(TextEvent::new(
                command.entity,
                &"Only alphabetic (a-z) characters are allowed.".to_string(),
            ));
            return Ok(());
        }

        let db_repo = world.resource::<DbInterface>();

        let user_exists = match db_repo.users.does_user_exist(username) {
            Ok(exists) => exists,
            Err(e) => {
                world.send_event(TextEvent::send_generic_error(command.entity));
                return Err(format!("Error while checking if user exists: {:?}", e));
            }
        };

        let mut user_sesh = world.get_mut::<UserSessionData>(command.entity).unwrap();

        user_sesh.username = username.to_string();

        if user_exists {
            user_sesh.status = UserStatus::NeedPassword;
            world.send_event(TextEvent::from_str(
                command.entity,
                "User account found. Please provide your password.",
            ));
        } else {
            user_sesh.status = UserStatus::CreatePassword;
            world.send_event(TextEvent::from_str(
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

        return false;
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let (entity, mut user_sesh) = world
            .query::<(Entity, &UserSessionData)>()
            .get_mut(world, command.entity)
            .unwrap();

        let password = command.full_command.clone();

        user_sesh.pwd = Some(password);
        user_sesh.status = UserStatus::ConfirmPassword;

        world.send_event(TextEvent::from_str(
            entity,
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

        return false;
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let (entity, mut user_sesh) = world
            .query::<(Entity, &UserSessionData)>()
            .get_mut(world, command.entity)
            .unwrap();

        let provided_password = command.full_command.clone();

        let db_repo = world.resource::<DbInterface>();

        let user_option = match db_repo
            .users
            .find_with_credentials(&user_sesh.username, &provided_password)
        {
            Ok(user) => user,
            Err(e) => {
                error!("Error while logging in user: {:?}", e);
                world.send_event(TextEvent::send_generic_error(entity));
                return Ok(());
            }
        };

        if user_option.is_none() {
            world.send_event(TextEvent::from_str(
                entity,
                "Looks like there's a problem with the password. Let's try again. What's your username?",
            ));
            user_sesh.username.clear();
            user_sesh.status = UserStatus::NeedUsername;
            return Ok(());
        }

        world.send_event(TextEvent::from_str(entity, "Thank you! Welcome back!\n\n"));

        let user = user_option.unwrap();
        user_sesh.status = UserStatus::LoggedIn;

        world.send_event(UserLoggedIn {
            entity,
            id: user.id,
        });
        Ok(())
    }
}
