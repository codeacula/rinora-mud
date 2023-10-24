use bevy::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection;
use diesel::r2d2::Pool;
use diesel::Connection;
use diesel::PgConnection;

use crate::prelude::*;

#[derive(Debug)]
struct TestTransaction;

impl CustomizeConnection<PgConnection, ::diesel::r2d2::Error> for TestTransaction {
    fn on_acquire(&self, conn: &mut PgConnection) -> Result<(), ::diesel::r2d2::Error> {
        conn.begin_test_transaction().unwrap();
        Ok(())
    }
}

fn get_connection_pool(conn_string: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(conn_string);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

fn get_test_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new("");
    Pool::builder()
        .connection_customizer(Box::new(TestTransaction))
        .build(manager)
        .expect("Could not build connection pool")
}

#[derive(Resource)]
pub struct DbInterface {
    pub characters: CharacterRepo,
    pub locations: LocationRepo,
    pub settings: SettingsRepo,
    pub users: UserRepo,
}

impl DbInterface {
    pub fn new(connection_string: String) -> Self {
        DbInterface {
            characters: CharacterRepo::new(get_connection_pool(&connection_string)),
            locations: LocationRepo::new(get_connection_pool(&connection_string)),
            settings: SettingsRepo::new(get_connection_pool(&connection_string)),
            users: UserRepo::new(get_connection_pool(&connection_string)),
        }
    }

    pub fn test() -> Self {
        DbInterface {
            characters: CharacterRepo::new(get_test_connection_pool()),
            locations: LocationRepo::new(get_test_connection_pool()),
            settings: SettingsRepo::new(get_test_connection_pool()),
            users: UserRepo::new(get_test_connection_pool()),
        }
    }
}
