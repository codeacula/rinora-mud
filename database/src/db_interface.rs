use bevy::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

use crate::prelude::*;

#[derive(Resource)]
pub struct DbInterface {
    pub characters: CharacterRepo,
    pub locations: LocationRepo,
    pub settings: SettingsRepo,
    pub users: UserRepo,
}

impl DbInterface {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        DbInterface {
            characters: CharacterRepo::new(pool.clone()),
            locations: LocationRepo::new(pool.clone()),
            settings: SettingsRepo::new(pool.clone()),
            users: UserRepo::new(pool.clone()),
        }
    }
}
