use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use prelude::*;

pub mod account;
pub mod being;
pub mod collections;
pub mod command;
pub mod content;
pub mod display;
pub mod helpers;
pub mod network;
pub mod settings;
pub mod status;
pub mod user;
pub mod world;

pub struct SharedPlugin;

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet, Copy)]
pub enum GameOrderSet {
    Network, // Ran first, so that all connections are handled and commands are sent to the command handlers
    Command, // The command step is meant to handle processing commands from the network layer
    Account, // The account section is a separate state from normal, and requires running first
    Pre, // The is for systems we want to run before the game, such as adding users to the game world
    Game, // This is where all the game systems are updated
    Post, // This is for systems that want to run after the world has processed but before cleanup
    Cleanup, // Cleanup is meant to handle stuff like storing removed characters and stuff
    Debug, // Debug is specifically so its output is sent before normal output
    Output, // Flush all output here
}

fn set_schedules(app: &mut App, stage: impl ScheduleLabel + Clone) {
    // Configure system sets
    let all_schedules = vec![
        GameOrderSet::Network,
        GameOrderSet::Command,
        GameOrderSet::Account,
        GameOrderSet::Pre,
        GameOrderSet::Game,
        GameOrderSet::Post,
        GameOrderSet::Cleanup,
        GameOrderSet::Debug,
        GameOrderSet::Output,
    ];

    for (i, current_set) in all_schedules.iter().enumerate() {
        if i != 0 {
            let previous_set = &all_schedules[i - 1];
            app.configure_set(stage.clone(), previous_set.before(current_set.to_owned()));
        }
        app.add_systems(stage.clone(), apply_deferred.in_set(current_set.to_owned()));
    }
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Configure system sets
        set_schedules(app, First);
        set_schedules(app, PreUpdate);
        set_schedules(app, Update);
        set_schedules(app, Last);

        // Account
        app.add_event::<CharacterCreatedEvent>()
            .add_event::<CharacterExistsEvent>()
            .add_event::<CharacterNameInvalidEvent>()
            .add_event::<CharacterNotFoundEvent>()
            .add_event::<CharacterSelectedEvent>()
            .add_event::<ConfirmPasswordDoesNotMatchEvent>()
            .add_event::<CreateCharacterEvent>()
            .add_event::<CreateCharacterSelectedEvent>()
            .add_event::<PasswordNotLongEnoughEvent>()
            .add_event::<UnableToLocateAccountEvent>()
            .add_event::<UserConfirmedPasswordEvent>()
            .add_event::<UserLoggedInEvent>()
            .add_event::<UserProvidedPasswordEvent>()
            .add_event::<UsernameDoesNotExistEvent>()
            .add_event::<UsernameExistsEvent>()
            .add_event::<UsernameInvalidEvent>()
            .add_event::<UsernameProvidedEvent>();

        // Entities
        app.add_event::<EntityEnteredRoomEvent>()
            .add_event::<EntityEnteredWorldEvent>()
            .add_event::<EntityLeftRoomEvent>()
            .add_event::<EntityLeftWorldEvent>();

        // Events
        app.add_event::<GenericErrorEvent>()
            .add_event::<ShowLoginScreenEvent>()
            .add_event::<ShowPromptEvent>()
            .add_event::<TextEvent>();
    }
}

pub mod prelude {
    pub use crate::account::*;
    pub use crate::being::*;
    pub use crate::collections::*;
    pub use crate::command::*;
    pub use crate::content::*;
    pub use crate::display::*;
    pub use crate::helpers::string::*;
    pub use crate::helpers::test::*;
    pub use crate::helpers::*;
    pub use crate::network::*;
    pub use crate::settings::*;
    pub use crate::status::*;
    pub use crate::user::*;
    pub use crate::world::*;

    pub use crate::GameOrderSet;
    pub use crate::SharedPlugin;

    pub use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap, utils::Uuid};
}
