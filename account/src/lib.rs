use bevy::{prelude::*, utils::HashMap};
use character_management::*;
use database::prelude::*;
use login_commands::*;
use shared::prelude::*;

mod character_management;
mod login_commands;

pub struct AccountPlugin;

/// Add keywords we can quickly check in the Commands module
fn add_expected_commands(
    mut expected_commands: ResMut<PossibleCommands>,
    mut command_list: ResMut<AccountCommands>,
) {
    expected_commands.0.push("acct".to_string());
    command_list.0.push(Box::new(UsernameProvided {}));
    command_list.0.push(Box::new(PasswordCreated {}));
    command_list.0.push(Box::new(PasswordProvided {}));
    command_list.0.push(Box::new(UserConfirmedPassword {}));
    command_list.0.push(Box::new(ProvideCharacterName {}));
    command_list.0.push(Box::new(SelectedCreateCharacter {}));
    command_list.0.push(Box::new(CharacterWasSelected {}));
}

pub fn get_login_screen(characters: &Vec<CharacterBundle>) -> String {
    let mut greeting = String::from("Your options:\n\n");

    greeting.push_str("  [{{15}}1{{7}}]: Create Character\n");
    greeting.push_str("  [{{15}}2{{7}}]: Delete Character\n");
    greeting.push_str("  [{{15}}3{{7}}]: Toggle Autologin\n\n");

    if characters.is_empty() {
        greeting.push_str("You currently have no characters.\n")
    } else {
        greeting.push_str("Your characters are:\n");

        for character in characters {
            greeting.push_str(&format!("  {}\n", character.info.shortname));
        }
    }

    greeting.push_str("\nSend a number command or which character you want to play.");
    greeting
}

/// When a user disconnects
pub fn handle_disconnect(
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

/// When someone first connects
pub fn handle_new_connections(
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

pub fn handle_user_login(
    mut query: Query<Entity>,
    mut events: EventReader<UserLoggedIn>,
    mut text_events_tx: EventWriter<TextEvent>,
    db_repo: Res<DbInterface>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let entity = query.get_mut(event.entity).unwrap();

        let found_user = match db_repo.users.get_by_id(event.id) {
            Ok(user) => user,
            Err(e) => {
                error!("Unable to fetch user after login: {:?}", e);
                text_events_tx.send(TextEvent::send_generic_error(entity));
                continue;
            }
        };

        let Some(user) = found_user else {
            error!("Unable to fetch user after login: No account returned!");
            text_events_tx.send(TextEvent::send_generic_error(entity));
            continue;
        };

        let characters = match db_repo.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!("Unable to fetch user's characters at login: {:?}", e);
                text_events_tx.send(TextEvent::from_str(
                    entity,
                    "There was an issue fetching your characters. Please disconnect and try again.",
                ));
                continue;
            }
        };

        text_events_tx.send(TextEvent::new(
            entity,
            &crate::get_login_screen(&characters),
        ));
        commands.entity(entity).insert(user);
    }
}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        let character_map = CharacterMap(HashMap::new());

        app.add_systems(Startup, add_expected_commands.in_set(GameOrderSet::Command))
            .insert_resource(character_map)
            .add_systems(
                Update,
                (handle_user_login, handle_disconnect, handle_new_connections)
                    .in_set(GameOrderSet::Command),
            );
    }
}
