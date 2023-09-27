use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

pub fn create_character(
    mut query: Query<(Entity, &User)>,
    mut events: EventReader<UserProvidedCharacterName>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let (entity, user) = query.get_mut(event.command.entity).unwrap();

        if event.command.parts.len() > 1 || !is_alphabetic(&event.command.keyword) {
            commands.add(SendText::new(
                entity,
                "Character names can only contain the letters A-Z, and only one word. Please try again.",
            ));
            continue;
        }
        let character_name = event.command.keyword.clone();

        let exists_res = db_repo
            .characters
            .does_character_exist(&character_name);

        if let Err(err) = exists_res {
            error!("Error checking if character exists: {:?}", err);
            commands.add(SendText::send_generic_error(entity));
            continue;
        }

        if exists_res.unwrap() {
            commands.add(SendText::new(
                entity,
                "That character already exists. Please try a different name.",
            ));
            continue;
        }

        if let Err(err) =
            db_repo
                .characters
                .create_character(&character_name, user)
        {
            error!("Error creating character for user: {:?}", err);
            commands.add(SendText::send_generic_error(entity));
            continue;
        }

        commands.add(SendText::new(
            entity,
            "Character created! You can now select them from the login screen",
        ));

        commands.add(TransitionUserToState {
            entity,
            state: UserStatus::LoggedIn,
        });

        let characters = match db_repo.characters.get_all_by_user( user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!(
                    "Unable to fetch user's characters after creating a character: {:?}",
                    e
                );
                commands.add(SendText::send_generic_error(entity));
                continue;
            }
        };

        commands.add(SendText::new(entity, &crate::get_login_screen(&characters)));
    }
}

pub fn process_character_deletion_requests(
    query: Query<(Entity, &Character)>,
    mut events: EventReader<DeleteCharacterEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    let mut entities_to_delete: Vec<String> = Vec::new();
    for event in events.iter() {
        if let Err(e) = db_repo
            .characters
            .delete_character(&event.name)
        {
            error!("There was an error deleting a user from the DB: {:?}", e);
            continue;
        }

        entities_to_delete.push(event.name.clone());
    }

    for (entity, character) in query.iter() {
        if entities_to_delete.contains(&character.shortname) {
            commands.entity(entity).remove::<Character>();
        }
    }
}

pub fn start_delete_character(
    mut query: Query<(Entity, &mut UserSessionData, &User)>,
    mut events: EventReader<UserProvidedCharacterToDelete>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let (entity, mut user_sesh, user) = query.get_mut(event.command.entity).unwrap();

        let character_name = event.command.keyword.clone();

        let query_res = db_repo
            .characters
            .get_character_by_name(&character_name);

        if let Err(err) = query_res {
            error!("Unable to get character by name: {:?}", err);
            continue;
        }

        let mut show_login_menu = false;
        let found_character = query_res.unwrap();

        if found_character.is_none() {
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
            let characters = match db_repo.characters.get_all_by_user(user.id) {
                Ok(characters) => characters,
                Err(e) => {
                    error!("Unable to fetch user's characters at login: {:?}", e);
                    commands.add(SendText::send_generic_error(entity));
                    continue;
                }
            };

            commands.add(SendText::new(entity, &crate::get_login_screen(&characters)));
            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::LoggedIn,
            });
            continue;
        }

        commands.add(SendText::new(
            entity,
            &format!(
                "{{{{9:0}}}}Are you sure you want to delete {}? Send their name again to confirm.",
                &character_name,
            ),
        ));

        commands.add(TransitionUserToState {
            entity,
            state: UserStatus::ConfirmDelete,
        });
        user_sesh.char_to_delete = Some(character_name);
    }
}

pub fn confirm_delete_character(
    mut query: Query<(Entity, &mut UserSessionData, &User)>,
    mut events: EventReader<UserConfirmedDeleteCharacter>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let (entity, mut user_sesh, user) = query.get_mut(event.command.entity).unwrap();

        if user_sesh.char_to_delete.is_none() {
            error!("Shouldn't have gotten to this state without a character provided");
            commands.add(SendText::send_generic_error(entity));
            continue;
        }

        let character_to_delete = user_sesh.char_to_delete.clone().unwrap();
        user_sesh.char_to_delete = None;
        let character_name = event.command.keyword.clone();

        let mut characters = match db_repo.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!(
                    "Unable to fetch user's characters at delete confirm: {:?}",
                    e
                );
                commands.add(SendText::send_generic_error(entity));
                continue;
            }
        };

        if character_to_delete.to_lowercase() != character_name.to_lowercase() {
            commands.add(SendText::new(
                entity,
                "The character names don't match. Aborting!",
            ));

            commands.add(TransitionUserToState {
                entity,
                state: UserStatus::LoggedIn,
            });

            commands.add(SendText::new(entity, &crate::get_login_screen(&characters)));
            continue;
        }

        characters.retain(|character| character.shortname != character_to_delete);
        commands.add(DeleteCharacter {
            name: character_to_delete,
        });
        commands.add(SendText::new(entity, "Alright! They'll be deleted!"));
        commands.add(SendText::new(entity, &crate::get_login_screen(&characters)));
        commands.add(TransitionUserToState {
            entity,
            state: UserStatus::LoggedIn,
        });
    }
}

pub fn process_loggedin_command(
    mut query: Query<(Entity, &User)>,
    mut events: EventReader<UserSelectedLoginOption>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let (entity, user) = query.get_mut(event.command.entity).unwrap();

        let characters = match db_repo.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!("Error fetching characters for login selection. {:?}", e);
                continue;
            }
        };

        // Wants to create a character
        if event.command.keyword == "1" {
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
        else if event.command.keyword == "2" {
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
        else if event.command.keyword == "3" {
            commands.add(SendText::new(entity, "You sent 3"));
            continue;
        } else if event.command.keyword == "exit" {
            commands.add(SendText::new(entity, "You sent exit"));
            continue;
        }

        let mut character_was_selected = false;

        // Wants to select a character
        for character in characters {
            if event.command.keyword.to_lowercase() == character.shortname.to_lowercase() {
                character_was_selected = true;
                commands.add(SendText::new(
                    entity,
                    &format!("You selected your character: {}", character.shortname),
                ));
                break;
            }
        }

        if !character_was_selected {
            commands.add(SendText::new(entity, "Invalid option. Try again!"));
        }
    }
}
