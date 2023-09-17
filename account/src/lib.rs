use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

pub struct AccountPlugin;

/// Add keywords we can quickly check in the Commands module
fn add_expected_commands(mut expected_commands: ResMut<PossibleCommands>) {
    expected_commands.0.push("acct".to_string());
}

/// When someone first connects
fn handle_new_connections(
    mut ev_new_connection: EventReader<NewConnectionEvent>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
) {
    for ev in ev_new_connection.iter() {
        ev_outgoing_text_events.send(TextEvent::from_str(
            ev.entity,
            "Please provide your username.",
        ));
    }
}

/// When a user disconnects
fn handle_disconnect(
    mut ev_disconnection_event: EventReader<DisconnectionEvent>,
    query: Query<&UserSessionData>,
    mut commands: Commands,
) {
    for ev in ev_disconnection_event.iter() {
        if let Ok(_user) = query.get(ev.entity) {
            commands.entity(ev.entity).despawn_recursive();
        } else {
            error!("User disconnected but no user component found");
        }
    }
}

/// The Command module dispatched a new AccountEvent. Let's handle it!
fn handle_account_event(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut incoming_account_events: EventReader<AccountEvent>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in incoming_account_events.iter() {
        let (entity, mut user_sesh) = query.get_mut(account_event.entity).unwrap();

        match user_sesh.status {
            UserStatus::CreatePassword => {
                let password = account_event.raw_command.clone();

                user_sesh.pwd = Some(password);
                user_sesh.status = UserStatus::ConfirmPassword;

                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "Excellent. Now, provide your password again for confirmation.",
                ));
            }
            UserStatus::ConfirmPassword => {
                if user_sesh.pwd.is_none() {
                    ev_outgoing_text_events.send(TextEvent::from_str(
                        entity,
                        "There was an error comparing your passwords. Email codeacula@codeacula.com for help.",
                    ));
                    error!("User got into ConfirmPassword state without having a password set in session!");
                    continue;
                }

                let original_password = user_sesh.pwd.as_ref().unwrap();
                let confirmation_password = &account_event.raw_command;

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
                    dbid: uuid,
                    username: user_sesh.username.clone(),
                });

                user_sesh.status = UserStatus::LoggedIn;

                ev_outgoing_text_events.send(TextEvent::from_str(
                    entity,
                    "Passwords match, account created! You are now logged in.",
                ));
            }
            UserStatus::NeedUsername => {
                if account_event.input.len() > 1 {
                    ev_outgoing_text_events.send(TextEvent::from_str(
                        entity,
                        "No spaces are allowed in usernames, only alphanumeric characters.",
                    ));
                    continue;
                }

                if !account_event
                    .get_keyword()
                    .chars()
                    .all(char::is_alphanumeric)
                {
                    ev_outgoing_text_events.send(TextEvent::from_str(
                        entity,
                        "Only alphanumeric characters are allowed.",
                    ));
                    continue;
                }

                let username = account_event.get_keyword();
                user_sesh.username = username;

                let user_exists = match db_repo.users.does_user_exist(&user_sesh.username) {
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
            UserStatus::NeedPassword => {
                let provided_password = account_event.raw_command.clone();

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

                let user = user_option.unwrap();
                user_sesh.status = UserStatus::LoggedIn;

                let mut greeting = String::from("Thank you! Welcome back!\n\nYour options:\n\n");

                greeting.push_str("  [{{15}}1{{7}}]: Create Character\n");
                greeting.push_str("  [{{15}}2{{7}}]: Delete Character\n");
                greeting.push_str("  [{{15}}3{{7}}]: Toggle Autologin\n\n");

                let characters = match db_repo.characters.get_all_by_user(user.dbid.clone()) {
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

                if characters.is_empty() {
                    greeting.push_str("You currently have no characters.\n")
                } else {
                    greeting.push_str("Your characters are:\n");

                    for character in characters {
                        greeting.push_str(&format!("  {}\n", character.name));
                    }
                }

                greeting.push_str("\nSend a number command or which character you want to play.");

                commands.entity(entity).insert(user);
                ev_outgoing_text_events.send(TextEvent::new(entity, &greeting));
            }
            _ => continue,
        }
    }
}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_expected_commands).add_systems(
            Update,
            (
                handle_disconnect,
                handle_new_connections,
                handle_account_event,
            ),
        );
    }
}
