use bevy::prelude::*;
use diesel::PgConnection;
use mongodb::{
    bson::doc,
    error::Error,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
    sync::Database,
};

use crate::prelude::{apply_migrations::apply_migrations, *};

#[derive(Resource)]
pub struct DbInterface {
    pub characters: CharacterRepo,
    pub client: PgConnection,
    pub rooms: RoomRepo,
    pub users: UserRepo,
    pub system: SystemRepo,
}

impl DbInterface {
    pub fn new(client: PgConnection) -> Self {
        let characters = CharacterRepo::new(&client);
        let rooms = RoomRepo::new(&database);
        let system = SystemRepo::new(&database);
        let users = UserRepo::new(&database);

        DbInterface {
            characters,
            client,
            rooms,
            system,
            users,
        }
    }

    pub fn disconnect(self) {}
}
