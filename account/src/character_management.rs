use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

pub fn create_character(
    mut query: Query<(Entity, &mut UserSessionData, Option<&User>)>,
    mut account_events: EventReader<AccountEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in account_events.iter() {
        let (entity, user_sesh, user_option) = match query.get_mut(account_event.entity) {
            Ok((entity, user_sesh, user)) => (entity, user_sesh, user),
            Err(_) => {
                // We're in here
                continue;
            }
        };

        if user_sesh.status != UserStatus::CreateCharacter {
            return;
        }

        let user = match user_option {
            None => {
                error!("No user object found when one was expected to be.");
                continue;
            }
            Some(user) => user,
        };

        let character_name = account_event.input[0].clone();

        if account_event.input.len() > 1 || !character_name.chars().all(char::is_alphabetic) {
            commands.add(SendText::new(
                entity,
                "Character names can only contain the letters A-Z. Please try again.",
            ));
            continue;
        }

        let exists_res = db_repo.characters.does_character_exist(&character_name);

        if let Err(err) = exists_res {
            error!("Error checking if character exists: {:?}", err);
            commands.add(SendText::new(
                entity,
                "There was an error creating your character.",
            ));
            continue;
        }

        if exists_res.unwrap() {
            commands.add(SendText::new(
                entity,
                "That character already exists. Please try again.",
            ));
            continue;
        }

        if let Err(err) = db_repo.characters.create_character(&character_name, user) {
            error!("Error creating character for user: {:?}", err);
            commands.add(SendText::new(
                entity,
                "There was an error creating your character.",
            ));
            continue;
        }

        commands.add(SendText::new(entity, "Character created!"));
    }
}

pub fn start_delete_character(
    mut query: Query<(Entity, &mut UserSessionData, Option<&User>)>,
    mut account_events: EventReader<AccountEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in account_events.iter() {
        let (entity, user_sesh, user_option) = match query.get_mut(account_event.entity) {
            Ok((entity, user_sesh, user)) => (entity, user_sesh, user),
            Err(_) => {
                // We're in here
                continue;
            }
        };

        if user_sesh.status != UserStatus::DeleteCharacter {
            continue;
        }

        let user = match user_option {
            None => {
                error!("No user object found when one was expected to be.");
                continue;
            }
            Some(user) => user,
        };

        let character_name = account_event.input[0].clone();

        let query_res = db_repo.characters.get_character_by_name(&character_name);

        if let Err(err) = query_res {
            error!("Unable to get character by name: {:?}", err);
            continue;
        }

        let mut show_login_menu = false;
        let found_character = query_res.unwrap();

        if let None = found_character {
            commands.add(SendText::new(
                entity,
                "Couldn't find a character by that name.",
            ));
            show_login_menu = true;
        }

        if let Some(character) = found_character {
            if character.user_id != user.id {
                commands.add(SendText::new(
                    entity,
                    "Couldn't find a character by that name.",
                ));
                show_login_menu = true;
            }
        }

        if show_login_menu {
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
            continue;
        }

        commands.add(SendText::new(
            entity,
            &format!(
                "{{{{8:0}}}}Are you sure you want to delete {}? Send their name again to confirm.",
                &character_name,
            ),
        ));
        commands.add(TransitionUserToState {
            entity,
            state: UserStatus::ConfirmDelete,
        });
    }
}

pub fn process_loggedin_command(
    mut query: Query<(Entity, &mut UserSessionData, Option<&User>)>,
    mut account_events: EventReader<AccountEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for account_event in account_events.iter() {
        let (entity, user_sesh, user_option) = match query.get_mut(account_event.entity) {
            Ok((entity, user_sesh, user)) => (entity, user_sesh, user),
            Err(_) => {
                // We're in here
                continue;
            }
        };

        if user_sesh.status != UserStatus::LoggedIn {
            return;
        }

        let user = match user_option {
            None => {
                error!("No user object found when one was expected to be.");
                continue;
            }
            Some(user) => user,
        };

        let characters = match db_repo.characters.get_all_by_user(&user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!("Error fetching characters for login selection. {:?}", e);
                continue;
            }
        };

        // Wants to create a character
        if account_event.raw_command == "1" {
            commands.add(SendText::new(
                entity,
                "{{11}}What would you like your character's name to be?",
            ));
            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::CreateCharacter,
            });
            continue;
        }
        // Wants to delete a character
        else if account_event.raw_command == "2" {
            commands.add(SendText::new(
                entity,
                "Provide the name of the character you'd like to delete.",
            ));
            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::DeleteCharacter,
            });
            continue;
        }
        // Wants to toggle auto login
        else if account_event.raw_command == "3" {
            commands.add(SendText::new(entity, "You sent 3"));
            continue;
        } else if account_event.raw_command == "exit" {
            commands.add(SendText::new(entity, "You sent exit"));
            continue;
        }

        // Wants to select a character
        for character in characters {
            if account_event.input[0].to_lowercase() == character.name.to_lowercase() {
                commands.add(SendText::new(
                    entity,
                    &format!("You selected your character: {}", character.name),
                ));
                continue;
            }
        }

        commands.add(SendText::new(entity, "Invalid option. Try again!"));
    }
}
