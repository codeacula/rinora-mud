use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

pub fn process_loggedin_command(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut login_option_selected_writer: EventReader<AccountEvent>,
) {
    for account_event in login_option_selected_writer.iter() {
        let (entity, user_sesh) = query.get_mut(account_event.entity).unwrap();

        if user_sesh.status != UserStatus::LoggedIn {
            return;
        }

        // Wants to create a character
        if account_event.raw_command == "1" {
        }
        // Wants to delete a character
        else if account_event.raw_command == "2" {
        }
        // Wants to toggle auto login
        else if account_event.raw_command == "3" {
        } else if account_event.raw_command == "exit" {
        }

        // Wants to select a character
    }
}
