use bevy::log::{Level, LogPlugin};
use bevy::utils::HashMap;
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
mod gmcp;
mod helpers;
mod models;
mod output;
mod resources;
mod stream_processor;
mod systems;

pub struct BaseRinoraPlugin;

impl Plugin for BaseRinoraPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        let mut command_list = GameCommands(HashMap::new());
        let connection_hashmap = HashMap::<Uuid, Entity>::new();
        let character_map = CharacterMap(HashMap::new());

        // Go ahead and make the vectors for all the statuses
        let statuses_to_add = vec![
            UserStatus::CreateCharacter,
            UserStatus::CreatePassword,
            UserStatus::ConfirmDelete,
            UserStatus::ConfirmPassword,
            UserStatus::DeleteCharacter,
            UserStatus::InGame,
            UserStatus::LoggedIn,
            UserStatus::NeedUsername,
            UserStatus::NeedPassword,
            UserStatus::ToggleAutologin,
        ];

        for status in statuses_to_add {
            command_list.0.insert(status, Vec::new());
        }

        app
            // System Plugins
            .add_plugins(LogPlugin {
                level: Level::DEBUG,
                filter: "debug,rinora_mud=debug".into(),
            })
            // Plugins
            .add_plugins((MinimalPlugins, SharedPlugin, DatabasePlugin, HelperPlugin))
            // Resources
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
            .add_systems(
                Startup,
                (add_expected_account_commands, add_character_commands)
                    .in_set(GameOrderSet::Command),
            )
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
            .add_systems(
                Update,
                (
                    create_character_selected,
                    check_username_and_transition_user,
                )
                    .in_set(GameOrderSet::Account),
            )
            .add_systems(Update, add_character_to_room.in_set(GameOrderSet::Game))
            .add_systems(
                Update,
                (
                    create_new_character,
                    create_new_user,
                    character_name_invalid,
                    character_was_created,
                    display_character_exists,
                    handle_generic_error,
                    passwords_do_not_match,
                    password_was_provided,
                    unable_to_locate_account,
                    username_exists,
                    username_does_not_exists,
                    username_invalid,
                )
                    .in_set(GameOrderSet::Account),
            )
            .add_systems(
                Update,
                (
                    log_character_into_game,
                    invalid_direction,
                    move_entity_to_room_via_event,
                )
                    .in_set(GameOrderSet::Pre),
            )
            .add_systems(
                Update,
                (log_character_into_room, process_entities_that_want_to_move)
                    .in_set(GameOrderSet::Game),
            )
            .add_systems(Update, (remove_logging_in_tags).in_set(GameOrderSet::Post))
            .add_systems(
                Update,
                (
                    display_character_entering_room,
                    display_character_logged_into_room.before(display_character_entering_room),
                    display_room_to_entity.after(display_character_entering_room),
                    prompt_for_character_name,
                    show_login_screen,
                )
                    .in_set(GameOrderSet::Output),
            )
            .add_systems(
                Last,
                (process_gmcp_requests, send_prompt_to_user).in_set(GameOrderSet::Output),
            )
            .add_systems(
                Last,
                (process_text_events_for_users, process_outgoing_data)
                    .in_set(GameOrderSet::Network),
            );
    }
}
