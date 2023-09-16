use bevy::prelude::*;
use std::env;

use crate::db_interface::DbInterface;

mod characters;
mod db_interface;
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

        app.insert_resource(repo);
    }
}

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::db_interface::*;
    pub use crate::users::*;
}
