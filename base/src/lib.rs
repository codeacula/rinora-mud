use accounts::AccountPlugin;
use bevy::log::{Level, LogPlugin};
use database::prelude::*;
use helper::*;
use networking::NetworkPlugin;
use output::{
    display_prompt::*, send_text_to_entity::send_text_to_entity,
    show_character_logging_in::show_character_logging_in,
};
use shared::prelude::*;
use systems::{
    handle_entities_talking::handle_entities_talking, run_user_commands::run_user_commands,
    send_heard_to_user::send_heard_to_user, spawn_character_in_room::spawn_character_in_room,
};

mod commands;
mod enums;
mod events;
mod helpers;
mod output;
mod systems;

pub struct BaseRinoraPlugin;

impl Plugin for BaseRinoraPlugin {
    fn build(&self, app: &mut App) {
        let account_comands: AccountCommands = AccountCommands(Vec::new());
        let mut game_commands: GameCommands = GameCommands(Vec::new());

        game_commands
            .0
            .push(Box::new(commands::say_command::SayCommand));

        app
            // System Plugins
            .add_plugins(LogPlugin {
                level: Level::DEBUG,
                filter: "debug,rinora_mud=debug".into(),
            })
            .insert_resource(account_comands)
            .insert_resource(game_commands)
            // Plugins
            .add_plugins((
                MinimalPlugins,
                SharedPlugin,
                DatabasePlugin,
                HelperPlugin,
                NetworkPlugin,
                AccountPlugin,
            ))
            .add_systems(First, (run_user_commands).in_set(GameOrderSet::Command))
            .add_systems(Update, (handle_entities_talking).in_set(GameOrderSet::Pre))
            .add_systems(Update, (spawn_character_in_room).in_set(GameOrderSet::Game))
            .add_systems(Update, (send_heard_to_user).in_set(GameOrderSet::Game))
            .add_systems(
                Update,
                (show_character_logging_in).in_set(GameOrderSet::Post),
            )
            .add_systems(First, (send_text_to_entity).in_set(GameOrderSet::Output))
            .add_systems(Update, (display_prompt).in_set(GameOrderSet::Output));
    }
}
