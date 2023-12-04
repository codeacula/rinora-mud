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
        let mut resource = app.world.get_resource_mut::<GameCommands>().unwrap();

        resource.0.push(Box::new(
            commands::selected_create_new_character::SelectedCreateNewCharacterCommand,
        ));

        resource.0.push(Box::new(
            commands::provides_user_name::ProvidesUserNameCommand,
        ));

        resource.0.push(Box::new(
            commands::provides_login_password::ProvidesLoginPasswordCommand,
        ));

        resource.0.push(Box::new(
            commands::new_account_password::NewAccountPasswordCommand,
        ));

        resource.0.push(Box::new(
            commands::new_character_name_provided::NewCharacterNameProvidedCommand,
        ));

        resource.0.push(Box::new(
            commands::pronouns_provided::PronounsProvidedCommand,
        ));

        resource.0.push(Box::new(
            commands::confirm_character_creation::ConfirmCharacterCreationCommand,
        ));

        app.add_event::<WelcomeUserEvent>();

        app.add_systems(Update, (log_out_users).in_set(GameOrderSet::Cleanup))
            .add_systems(Update, (show_welcome_menu).in_set(GameOrderSet::Output));
    }
}
