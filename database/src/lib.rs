use bevy::prelude::*;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use importers::rooms::add_rooms_to_world;
use std::env;

use crate::db_interface::DbInterface;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

mod characters;
mod db_interface;
mod importers;
mod rooms;
mod system;
mod users;

pub struct DatabasePlugin;

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(String::from(default))
}

fn get_database() -> String {
    get_env("DB_DATABASE", "rinoramud")
}

pub struct PgConnectionWrapper {
    conn: PgConnection,
}

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        let host_string = get_env("DB_CONN_STRING", "postgresql://dev:dev@localhost/rinoramud");

        info!("Attempting to connect to database. {}", &host_string);
        let pg_conn = PgConnection::establish(&host_string).unwrap();

        info!("Running migrations {}", &host_string);
        pg_conn.run_pending_migrations(MIGRATIONS);

        let repo = DbInterface::new(pg_conn);

        repo.migrate().unwrap();

        app.insert_resource(repo)
            .add_systems(Startup, add_rooms_to_world);
    }
}

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::db_interface::*;
    pub use crate::importers::*;
    pub use crate::rooms::*;
    pub use crate::system::*;
    pub use crate::users::*;
}
