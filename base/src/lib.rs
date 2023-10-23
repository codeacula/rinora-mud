use bevy::log::{Level, LogPlugin};
use database::prelude::*;
use events::*;
use helper::*;
use output::*;
use prelude::*;
use resources::*;
use shared::prelude::*;
use systems::prelude::*;

mod commands;
mod enums;
mod events;
mod helpers;
mod models;
mod output;
mod resources;
mod systems;

pub struct BaseRinoraPlugin;

impl Plugin for BaseRinoraPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        let account_commands = AccountCommands(Vec::new());
        let command_list = GameCommands(Vec::new());
        let connection_hashmap = HashMap::<Uuid, Entity>::new();
        let character_map = CharacterMap(HashMap::new());

        app
            // System Plugins
            .add_plugins(LogPlugin {
                level: Level::DEBUG,
                filter: "debug,rinora_mud=debug".into(),
            })
            // Resources
            .insert_resource(account_commands)
            .insert_resource(character_map)
            .insert_resource(command_list)
            .insert_resource(NetworkInfo {
                connection_to_entity: connection_hashmap,
            })
            // Events
            .add_event::<NewConnectionEvent>()
            .add_event::<InputReceivedEvent>()
            .add_event::<DisconnectionEvent>()
            .add_event::<GmcpReceivedEvent>()
            // Systems
            .add_systems(Startup, start_listening.in_set(GameOrderSet::Network))
            .add_systems(Startup, add_expected_commands.in_set(GameOrderSet::Command))
            .add_systems(
                First,
                transfer_from_server_to_game.in_set(GameOrderSet::Network),
            )
            .add_systems(
                PreUpdate,
                process_incoming_commands.in_set(GameOrderSet::Command),
            )
            .add_systems(
                Update,
                (handle_user_login, handle_disconnect, handle_new_connections)
                    .in_set(GameOrderSet::Command),
            )
            .add_systems(Update, add_character_to_room.in_set(GameOrderSet::Game))
            .add_systems(
                Update,
                (
                    character_name_invalid,
                    character_was_created,
                    display_character_exists,
                    handle_generic_error,
                )
                    .in_set(GameOrderSet::Post),
            )
            .add_systems(
                Update,
                (
                    display_character_entering_room,
                    display_character_logged_into_room,
                    display_room_to_user,
                    show_login_screen,
                    send_prompt_to_user,
                )
                    .in_set(GameOrderSet::Output),
            )
            .add_systems(
                Last,
                (process_text_events_for_users, process_outgoing_data)
                    .in_set(GameOrderSet::Network),
            )
            // Plugins
            .add_plugins((MinimalPlugins, SharedPlugin, DatabasePlugin, HelperPlugin));
    }
}
