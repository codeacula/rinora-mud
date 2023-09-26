use bevy::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

use crate::prelude::UserRepo;

fn get_connection_pool(conn_string: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(conn_string);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[derive(Resource)]
pub struct DbInterface {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
    connection_string: String,
    users: UserRepo,
}

impl DbInterface {
    pub fn new(connection_string: String) -> Self {
        DbInterface {
            connection_pool: get_connection_pool(connection_string),
            connection_string: connection_string.clone(),
            users: UserRepo::new(get_connection_pool(connection_string)),
        }
    }
}
