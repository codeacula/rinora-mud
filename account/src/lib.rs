use bevy::prelude::*;
use shared::prelude::*;

mod character_management;
mod connection_handlers;
mod login_workflow;

pub struct AccountPlugin;

/// Add keywords we can quickly check in the Commands module
fn add_expected_commands(mut expected_commands: ResMut<PossibleCommands>) {
    expected_commands.0.push("acct".to_string());
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
            greeting.push_str(&format!("  {}\n", character.name));
        }
    }

    greeting.push_str("\nSend a number command or which character you want to play.");
    greeting
}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_expected_commands).add_systems(
            Update,
            (
                connection_handlers::handle_disconnect,
                connection_handlers::handle_new_connections,
                login_workflow::handle_user_login,
                login_workflow::user_provided_username,
                login_workflow::user_create_password,
                login_workflow::user_confirmed_password,
                login_workflow::user_provided_password,
                character_management::process_loggedin_command,
                character_management::create_character,
                character_management::start_delete_character,
            ),
        );
    }
}
