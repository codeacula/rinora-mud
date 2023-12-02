use events::*;
use output::{
    ask_user_for_new_account_password::*, confirm_account_password::confirm_account_password,
    welcome_user_on_login::welcome_user_on_login,
};
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

        app.add_event::<AccountNotFoundEvent>()
            .add_event::<CreatingNewAccountEvent>()
            .add_event::<InvalidUsernameFormatEvent>()
            .add_event::<LoggingInEvent>()
            .add_event::<WelcomeUserEvent>();

        app.add_systems(
            Update,
            (
                ask_user_for_new_account_password,
                confirm_account_password,
                welcome_user_on_login,
            )
                .in_set(GameOrderSet::Output),
        );
    }
}
