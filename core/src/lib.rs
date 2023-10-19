use bevy::log::{Level, LogPlugin};
use database::prelude::*;
use helper::HelperPlugin;
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

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let account_commands = AccountCommands(Vec::new());
        let command_list = GameCommands(Vec::new());
        let connection_hashmap = HashMap::<Uuid, Entity>::new();
        let character_map = CharacterMap(HashMap::new());

        app.add_systems(Startup, add_expected_commands.in_set(GameOrderSet::Command))
            .insert_resource(character_map)
            .add_systems(
                Update,
                (handle_user_login, handle_disconnect, handle_new_connections)
                    .in_set(GameOrderSet::Command),
            );

        app.insert_resource(NetworkInfo {
            connection_to_entity: connection_hashmap,
        })
        .add_event::<NewConnectionEvent>()
        .add_event::<InputReceivedEvent>()
        .add_event::<DisconnectionEvent>()
        .add_event::<GmcpReceivedEvent>()
        .add_systems(Startup, start_listening.in_set(GameOrderSet::Network))
        .add_systems(
            First,
            transfer_from_server_to_game.in_set(GameOrderSet::Network),
        )
        .add_systems(
            Last,
            (process_text_events_for_users, process_outgoing_data).in_set(GameOrderSet::Network),
        )
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            filter: "debug,rinora_mud=debug".into(),
        })
        .add_systems(Update, add_character_to_room.in_set(GameOrderSet::Game))
        .add_systems(
            Update,
            (
                display_character_entering_room,
                display_character_logged_into_room,
                display_room_to_user,
            )
                .in_set(GameOrderSet::Output),
        )
        .insert_resource(account_commands)
        .insert_resource(command_list)
        .add_systems(
            PreUpdate,
            process_incoming_commands.in_set(GameOrderSet::Command),
        )
        .add_plugins((MinimalPlugins, SharedPlugin, DatabasePlugin, HelperPlugin));
    }
}
