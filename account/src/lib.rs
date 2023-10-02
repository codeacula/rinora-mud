use bevy::prelude::*;
use commands::UsernameProvided;
use shared::prelude::*;

mod character_management;
mod commands;
mod connection_handlers;
mod login_workflow;

pub struct AccountPlugin;

/// Add keywords we can quickly check in the Commands module
fn add_expected_commands(
    mut expected_commands: ResMut<PossibleCommands>,
    mut command_list: ResMut<GameCommands>,
) {
    expected_commands.0.push("acct".to_string());
    command_list.0.push(Box::new(UsernameProvided {}))
}

pub fn get_login_screen(characters: &Vec<Character>) -> String {
    let mut greeting = String::from("Your options:\n\n");

    greeting.push_str("  [{{15}}1{{7}}]: Create Character\n");
    greeting.push_str("  [{{15}}2{{7}}]: Delete Character\n");
    greeting.push_str("  [{{15}}3{{7}}]: Toggle Autologin\n\n");

    if characters.is_empty() {
        greeting.push_str("You currently have no characters.\n")
    } else {
        greeting.push_str("Your characters are:\n");

        for character in characters {
            greeting.push_str(&format!("  {}\n", character.shortname));
        }
    }

    greeting.push_str("\nSend a number command or which character you want to play.");
    greeting
}

pub fn route_commands_to_systems(
    mut query: Query<&UserSessionData>,
    mut account_events: EventReader<AccountEvent>,
    mut username_provided_events: EventWriter<UserProvidedUsername>,
    mut password_provided_events: EventWriter<UserProvidedPassword>,
    mut password_created_events: EventWriter<UserCreatedPassword>,
    mut password_confirmed_events: EventWriter<UserConfirmedPassword>,
    mut user_selected_login_option_event: EventWriter<UserSelectedLoginOption>,
    mut user_provided_character_name_event: EventWriter<UserProvidedCharacterName>,
    mut start_delete_character_event: EventWriter<UserProvidedCharacterToDelete>,
    mut confirm_delete_character_event: EventWriter<UserConfirmedDeleteCharacter>,
) {
    for account_event in account_events.iter() {
        let user_sesh = match query.get_mut(account_event.entity) {
            Ok(user_sesh) => user_sesh,
            Err(_) => {
                error!("Made it to the account commands parser without a user session");
                continue;
            }
        };

        let command = account_event.command.clone();

        match user_sesh.status {
            UserStatus::NeedUsername => {
                username_provided_events.send(UserProvidedUsername { command })
            }
            UserStatus::NeedPassword => {
                password_provided_events.send(UserProvidedPassword { command })
            }
            UserStatus::CreatePassword => {
                password_created_events.send(UserCreatedPassword { command })
            }
            UserStatus::ConfirmPassword => {
                password_confirmed_events.send(UserConfirmedPassword { command })
            }
            UserStatus::LoggedIn => {
                user_selected_login_option_event.send(UserSelectedLoginOption { command })
            }
            UserStatus::CreateCharacter => {
                user_provided_character_name_event.send(UserProvidedCharacterName { command })
            }
            UserStatus::DeleteCharacter => {
                start_delete_character_event.send(UserProvidedCharacterToDelete { command })
            }
            UserStatus::ConfirmDelete => {
                confirm_delete_character_event.send(UserConfirmedDeleteCharacter { command })
            }
            UserStatus::ToggleAutologin => todo!("Still need to do this"),
            UserStatus::InGame => {
                // Should be impossible to get here
                error!("User somehow fell into InGame during account command");
                continue;
            }
        }
    }
}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_expected_commands)
            .add_systems(First, route_commands_to_systems)
            .add_systems(
                Update,
                (
                    connection_handlers::handle_disconnect,
                    connection_handlers::handle_new_connections,
                    login_workflow::handle_user_login,
                    login_workflow::user_provided_username,
                    login_workflow::user_create_password,
                    login_workflow::user_confirmed_password,
                    login_workflow::user_provided_password,
                    character_management::confirm_delete_character,
                    // character_management::process_loggedin_command,
                    character_management::create_character,
                    character_management::start_delete_character,
                ),
            )
            .add_systems(
                Last,
                character_management::process_character_deletion_requests,
            );
    }
}
