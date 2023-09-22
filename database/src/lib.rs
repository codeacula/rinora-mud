use bevy::prelude::*;
use importers::rooms::add_rooms_to_world;
use std::env;

use crate::db_interface::DbInterface;

mod characters;
mod db_interface;
mod importers;
mod migrations;
mod rooms;
mod system;
mod users;

pub struct DatabasePlugin;

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(String::from(default))
}

fn get_database() -> String {
    get_env("MONGODB_DATABASE", "rinoramud")
}

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        let host_string = get_env("MONGODB_CONN_STRING", "mongodb://localhost");
        let database_name = get_database();

        let repo = DbInterface::new(&host_string, &database_name);

        info!("Attempting to connect to database. {}", &host_string);

        repo.ping().unwrap();

        info!("Running migrations {}", &host_string);
        repo.migrate().unwrap();

        app.insert_resource(repo)
            .add_systems(Startup, add_rooms_to_world);
    }
}

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::db_interface::*;
    pub use crate::importers::*;
    pub use crate::migrations::*;
    pub use crate::rooms::*;
    pub use crate::system::*;
    pub use crate::users::*;
}
