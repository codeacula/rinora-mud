use events::WelcomeUserEvent;
use output::show_welcome_menu::show_welcome_menu;
use shared::prelude::*;
use system::log_out_users::log_out_users;

pub struct AccountPlugin;

mod commands;
mod components;
mod events;
mod output;
mod system;

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        let mut command_list = app.world.get_resource_mut::<AccountCommands>().unwrap();

        command_list.0.push(Box::new(
            commands::selected_create_new_character::SelectedCreateNewCharacterCommand,
        ));

        command_list.0.push(Box::new(
            commands::provides_user_name::ProvidesUserNameCommand,
        ));

        command_list.0.push(Box::new(
            commands::provides_login_password::ProvidesLoginPasswordCommand,
        ));

        command_list.0.push(Box::new(
            commands::new_account_password::NewAccountPasswordCommand,
        ));

        command_list.0.push(Box::new(
            commands::new_character_name_provided::NewCharacterNameProvidedCommand,
        ));

        command_list.0.push(Box::new(
            commands::pronouns_provided::PronounsProvidedCommand,
        ));

        command_list.0.push(Box::new(
            commands::confirm_character_creation::ConfirmCharacterCreationCommand,
        ));

        command_list
            .0
            .push(Box::new(commands::select_character::SelectCharacterCommand));

        app.add_event::<WelcomeUserEvent>();

        app.add_systems(Update, (log_out_users).in_set(GameOrderSet::Cleanup))
            .add_systems(Update, (show_welcome_menu).in_set(GameOrderSet::Output));
    }
}
