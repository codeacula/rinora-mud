use bevy::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

use crate::prelude::*;

fn get_connection_pool(conn_string: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(conn_string);
    Pool::builder()
        .test_on_check_out(true)
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
    pub fn new(connection_string: &str) -> Self {
        DbInterface {
            characters: CharacterRepo::new(get_connection_pool(connection_string)),
            locations: LocationRepo::new(get_connection_pool(connection_string)),
            settings: SettingsRepo::new(get_connection_pool(connection_string)),
            users: UserRepo::new(get_connection_pool(connection_string)),
        }
    }
}
