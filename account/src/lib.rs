use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

mod connection_handlers;

pub struct AccountPlugin;

/// Add keywords we can quickly check in the Commands module
fn add_expected_commands(mut expected_commands: ResMut<PossibleCommands>) {
    expected_commands.0.push("acct".to_string());
}

fn get_login_screen(characters: &Vec<Character>) -> String {
    let mut greeting = String::from("Your options:\n\n");

    greeting.push_str("  [{{15}}1{{7}}]: Create Character\n");
    greeting.push_str("  [{{15}}2{{7}}]: Delete Character\n");
    greeting.push_str("  [{{15}}3{{7}}]: Toggle Autologin\n\n");

    if characters.is_empty() {
        greeting.push_str("You currently have no characters.\n")
    } else {
        greeting.push_str("Your characters are:\n");

        for character in characters {
            greeting.push_str(&format!("  {}\n", character.name));
        }
    }

    greeting.push_str("\nSend a number command or which character you want to play.");
    greeting
}

fn handle_user_login(
    mut query: Query<Entity>,
    mut user_login_events: EventReader<UserLoggedIn>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in user_login_events.iter() {
        let entity = query.get_mut(account_event.entity).unwrap();
        let found_user = match db_repo.users.get_by_uuid(&account_event.uuid) {
            Ok(user) => user,
            Err(e) => {
                error!("Unable to fetch user after login: {:?}", e);
                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "There was an issue fetching your characters. Please disconnect and try again.",
                ));
                continue;
            }
        };

        let Some(user) = found_user else {
            error!("Unable to fetch user after login: No account returned!");
            ev_outgoing_text_events.send(TextEvent::from_str(
                entity,
                "There was an issue fetching your characters. Please disconnect and try again.",
            ));
            continue;
        };

        let characters = match db_repo.characters.get_all_by_user(&user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!("Unable to fetch user's characters at login: {:?}", e);
                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "There was an issue fetching your characters. Please disconnect and try again.",
                ));
                continue;
            }
        };

        ev_outgoing_text_events.send(TextEvent::new(entity, &get_login_screen(&characters)));
        commands.entity(entity).insert(user);
        get_login_screen(&characters);
    }
}

fn user_confirmed_password(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut user_confirmed_password_events: EventReader<UserConfirmedPassword>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
    mut user_logged_in_writer: EventWriter<UserLoggedIn>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in user_confirmed_password_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();
        if user_sesh.pwd.is_none() {
            ev_outgoing_text_events.send(TextEvent::from_str(
            entity,
            "There was an error comparing your passwords. Email codeacula@codeacula.com for help.",
        ));
            error!("User got into ConfirmPassword state without having a password set in session!");
            continue;
        }

        let original_password = user_sesh.pwd.as_ref().unwrap();
        let confirmation_password = &account_event.password;

        if original_password != confirmation_password {
            ev_outgoing_text_events.send(TextEvent::from_str(
                entity,
                "Your passwords don't match, let's try again. What password do you want?",
            ));
            user_sesh.status = UserStatus::CreatePassword;
            return;
        }

        let uuid = match db_repo
            .users
            .create_user(&user_sesh.username, confirmation_password)
        {
            Ok(uuid) => uuid,
            Err(e) => {
                ev_outgoing_text_events.send(TextEvent::from_str(
                entity,
                "There was an error creating your account. Email codeacula@codeacula.com for help.",
            ));
                error!("Unable to create user: {:?}", e);
                continue;
            }
        };

        commands.entity(entity).insert(User {
            autologin: String::from(""),
            id: uuid.clone(),
            username: user_sesh.username.clone(),
        });

        ev_outgoing_text_events.send(TextEvent::from_str(
            entity,
            "Passwords match, account created! You are now logged in.\n\n",
        ));
        user_sesh.status = UserStatus::LoggedIn;

        user_logged_in_writer.send(UserLoggedIn { entity, uuid });
    }
}

fn user_create_password(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut incoming_account_events: EventReader<UserCreatedPassword>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
) {
    for account_event in incoming_account_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();
        let password = account_event.password.clone();

        user_sesh.pwd = Some(password);
        user_sesh.status = UserStatus::ConfirmPassword;

        ev_outgoing_text_events.send(TextEvent::from_str(
            entity,
            "Excellent. Now, provide your password again for confirmation.",
        ));
    }
}

fn user_provided_password(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut user_provided_password_events: EventReader<UserProvidedPassword>,
    mut user_logged_in_writer: EventWriter<UserLoggedIn>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
    db_repo: Res<DbInterface>,
) {
    for account_event in user_provided_password_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();
        let provided_password = account_event.password.clone();

        let user_option = match db_repo
            .users
            .find_with_credentials(&user_sesh.username, &provided_password)
        {
            Ok(user) => user,
            Err(e) => {
                error!("Error while logging in user: {:?}", e);
                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "There was an error checking for your account.",
                ));
                continue;
            }
        };

        if user_option.is_none() {
            ev_outgoing_text_events.send(TextEvent::from_str(
                entity,
                "Looks like there's a problem with the password. Let's try again. What's your username?",
            ));
            user_sesh.username.clear();
            user_sesh.status = UserStatus::NeedUsername;
            continue;
        }

        ev_outgoing_text_events.send(TextEvent::from_str(entity, "Thank you! Welcome back!\n\n"));
        let user = user_option.unwrap();
        user_sesh.status = UserStatus::LoggedIn;

        user_logged_in_writer.send(UserLoggedIn {
            entity,
            uuid: user.id,
        });
    }
}

fn user_provided_username(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut incoming_account_events: EventReader<UserProvidedUsername>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
    db_repo: Res<DbInterface>,
) {
    for account_event in incoming_account_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();

        let username = String::from(account_event.username.clone().trim());

        if !username.chars().all(char::is_alphanumeric) {
            ev_outgoing_text_events.send(TextEvent::from_str(
                entity,
                "Only alphanumeric characters are allowed.",
            ));
            continue;
        }

        let user_exists = match db_repo.users.does_user_exist(&username) {
            Ok(exists) => exists,
            Err(e) => {
                error!("Error while checking if user exists: {:?}", e);
                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "There was an error checking for your account.",
                ));
                continue;
            }
        };

        user_sesh.username = username;

        if user_exists {
            ev_outgoing_text_events.send(TextEvent::from_str(
                entity,
                "User account found. Please provide your password.",
            ));
            user_sesh.status = UserStatus::NeedPassword;
        } else {
            ev_outgoing_text_events.send(TextEvent::from_str(
                entity,
                "Welcome, new user! What should your password be?",
            ));
            user_sesh.status = UserStatus::CreatePassword;
        }
    }
}

fn process_account_commands(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut incoming_account_events: EventReader<AccountEvent>,
    mut user_provided_username_writer: EventWriter<UserProvidedUsername>,
    mut user_create_password_writer: EventWriter<UserCreatedPassword>,
    mut user_confirm_password_writer: EventWriter<UserConfirmedPassword>,
    mut user_provided_password_writer: EventWriter<UserProvidedPassword>,
    mut login_option_selected_writer: EventWriter<LoginOptionSelected>,
) {
    for account_event in incoming_account_events.iter() {
        let (entity, user_sesh) = query.get_mut(account_event.entity).unwrap();
        let command = account_event.raw_command.clone();

        match user_sesh.status {
            UserStatus::NeedUsername => user_provided_username_writer.send(UserProvidedUsername {
                entity,
                username: command,
            }),
            UserStatus::CreatePassword => user_create_password_writer.send(UserCreatedPassword {
                entity,
                password: command,
            }),
            UserStatus::ConfirmPassword => {
                user_confirm_password_writer.send(UserConfirmedPassword {
                    entity,
                    password: command,
                })
            }
            UserStatus::NeedPassword => user_provided_password_writer.send(UserProvidedPassword {
                entity,
                password: command,
            }),

            UserStatus::LoggedIn => login_option_selected_writer.send(LoginOptionSelected {
                entity: entity,
                option: command,
            }),

            UserStatus::InGame => {
                continue;
            }
        }
    }
}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_expected_commands)
            .add_systems(First, process_account_commands)
            .add_systems(
                Update,
                (
                    connection_handlers::handle_disconnect,
                    connection_handlers::handle_new_connections,
                    handle_user_login,
                    user_provided_username,
                    user_create_password,
                    user_confirmed_password,
                    user_provided_password,
                ),
            );
    }
}
