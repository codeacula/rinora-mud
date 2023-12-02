use events::*;
use shared::prelude::*;

pub struct AccountPlugin;

mod commands;
mod components;
mod events;

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        let mut resource = app.world.get_resource_mut::<GameCommands>().unwrap();

        resource.0.push(Box::new(
            commands::provides_user_name::ProvidesUserNameCommand,
        ));

        app.add_event::<InvalidUsernameFormatEvent>();
    }
}
