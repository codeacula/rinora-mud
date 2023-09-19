use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

pub fn handle_user_login(
    mut query: Query<Entity>,
    mut user_login_events: EventReader<UserLoggedIn>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in user_login_events.iter() {
        let entity = query.get_mut(account_event.entity).unwrap();

        let found_user = match db_repo.users.get_by_uuid(&account_event.uuid) {
            Ok(user) => user,
            Err(e) => {
                error!("Unable to fetch user after login: {:?}", e);
                commands.add(SendText::new(
                    entity,
                    "There was an issue fetching your account. Please disconnect and try again.",
                ));
                continue;
            }
        };

        let Some(user) = found_user else {
            error!("Unable to fetch user after login: No account returned!");
            commands.add(SendText::new(
                entity,
                "There was an issue fetching your account. Please disconnect and try again.",
            ));
            continue;
        };

        let characters = match db_repo.characters.get_all_by_user(&user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!("Unable to fetch user's characters at login: {:?}", e);
                commands.add(SendText::new(
                    entity,
                    "There was an issue fetching your characters. Please disconnect and try again.",
                ));
                continue;
            }
        };

        commands.add(SendText::new(entity, &crate::get_login_screen(&characters)));
        commands.entity(entity).insert(user);
    }
}

pub fn user_confirmed_password(
    mut query: Query<(Entity, &UserSessionData)>,
    mut account_events: EventReader<AccountEvent>,
    mut user_logged_in_writer: EventWriter<UserLoggedIn>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in account_events.iter() {
        let (entity, user_sesh) = query.get_mut(account_event.entity).unwrap();

        if user_sesh.status != UserStatus::ConfirmPassword {
            continue;
        }

        if user_sesh.pwd.is_none() {
            error!("User got into ConfirmPassword state without having a password set in session!");
            commands.add(SendText::new(
                entity,
                "There was an error comparing your passwords. Email codeacula@codeacula.com for help.",
            ));
            continue;
        }

        let original_password = user_sesh.pwd.as_ref().unwrap();
        let confirmation_password = &account_event.raw_command;

        if original_password != confirmation_password {
            commands.add(SendText::new(
                entity,
                "Your passwords don't match, let's try again. What password do you want?",
            ));

            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::CreatePassword,
            });
            return;
        }

        let uuid = match db_repo
            .users
            .create_user(&user_sesh.username, confirmation_password)
        {
            Ok(uuid) => uuid,
            Err(e) => {
                error!("Unable to create user: {:?}", e);
                commands.add(SendText::new(
                    entity,
                    "There was an error creating your account. Email codeacula@codeacula.com for help.",
                ));
                continue;
            }
        };

        commands.entity(entity).insert(User {
            autologin: String::from(""),
            id: uuid.clone(),
            username: user_sesh.username.clone(),
        });

        commands.add(SendText::new(
            entity,
            "Passwords match, account created! You are now logged in.\n\n",
        ));

        commands.add(TransitionUserToState {
            entity,
            state: UserStatus::LoggedIn,
        });

        user_logged_in_writer.send(UserLoggedIn { entity, uuid });
    }
}

pub fn user_create_password(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut account_events: EventReader<AccountEvent>,
    mut commands: Commands,
) {
    for account_event in account_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();

        if user_sesh.status != UserStatus::CreatePassword {
            continue;
        }

        let password = account_event.raw_command.clone();

        user_sesh.pwd = Some(password);

        commands.add(TransitionUserToState {
            entity,
            state: UserStatus::ConfirmPassword,
        });

        commands.add(SendText::new(
            entity,
            "Excellent. Now, provide your password again for confirmation.",
        ));
    }
}

pub fn user_provided_password(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut account_events: EventReader<AccountEvent>,
    mut user_logged_in_writer: EventWriter<UserLoggedIn>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in account_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();

        if user_sesh.status != UserStatus::NeedPassword {
            continue;
        }

        let provided_password = account_event.raw_command.clone();

        let user_option = match db_repo
            .users
            .find_with_credentials(&user_sesh.username, &provided_password)
        {
            Ok(user) => user,
            Err(e) => {
                error!("Error while logging in user: {:?}", e);

                commands.add(SendText::new(
                    entity,
                    "There was an error checking for your account.",
                ));
                continue;
            }
        };

        if user_option.is_none() {
            commands.add(SendText::new(
                entity,
                "Looks like there's a problem with the password. Let's try again. What's your username?",
            ));
            user_sesh.username.clear();
            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::NeedUsername,
            });
            continue;
        }

        commands.add(SendText::new(entity, "Thank you! Welcome back!\n\n"));

        let user = user_option.unwrap();

        commands.add(TransitionUserToState {
            entity,
            state: UserStatus::LoggedIn,
        });

        user_logged_in_writer.send(UserLoggedIn {
            entity,
            uuid: user.id,
        });
    }
}

pub fn user_provided_username(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut account_events: EventReader<AccountEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in account_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();

        if user_sesh.status != UserStatus::NeedUsername {
            continue;
        }

        let username = account_event.input[0].clone();

        if !username.chars().all(char::is_alphanumeric) {
            commands.add(SendText::new(
                entity,
                "Only alphanumeric characters are allowed.",
            ));
            continue;
        }

        let user_exists = match db_repo.users.does_user_exist(&username) {
            Ok(exists) => exists,
            Err(e) => {
                error!("Error while checking if user exists: {:?}", e);
                commands.add(SendText::new(
                    entity,
                    "There was an error checking for your account.",
                ));
                continue;
            }
        };

        user_sesh.username = username;

        if user_exists {
            commands.add(SendText::new(
                entity,
                "User account found. Please provide your password.",
            ));
            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::NeedPassword,
            });
        } else {
            commands.add(SendText::new(
                entity,
                "Welcome, new user! What should your password be?",
            ));
            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::CreatePassword,
            });
        }
    }
}
