use bevy::prelude::*;
use diesel::{Connection, PgConnection};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::env;

use crate::db_interface::DbInterface;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

mod characters;
mod db_interface;
mod locations;
mod schema;
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
        let mut pg_conn = PgConnection::establish(&host_string).unwrap();

        info!("Running migrations {}", &host_string);
        pg_conn.run_pending_migrations(MIGRATIONS).unwrap();

        let repo = DbInterface::new(host_string);
        app.insert_resource(repo);
    }
}

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::db_interface::*;
    pub use crate::locations::*;
    pub use crate::system::*;
    pub use crate::users::*;
}
