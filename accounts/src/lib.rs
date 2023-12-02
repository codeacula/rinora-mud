use events::WelcomeUserEvent;
use output::show_welcome_menu::show_welcome_menu;
use shared::prelude::*;

pub struct AccountPlugin;

mod commands;
mod components;
mod events;
mod output;

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        let mut resource = app.world.get_resource_mut::<GameCommands>().unwrap();

        resource.0.push(Box::new(
            commands::provides_user_name::ProvidesUserNameCommand,
        ));

        resource.0.push(Box::new(
            commands::provides_login_password::ProvidesLoginPassword,
        ));

        resource
            .0
            .push(Box::new(commands::new_account_password::NewAccountPassword));

        app.add_event::<WelcomeUserEvent>();

        app.add_systems(Update, (show_welcome_menu).in_set(GameOrderSet::Output));
    }
}
