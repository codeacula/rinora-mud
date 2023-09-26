use diesel::PgConnection;

use crate::prelude::*;

pub struct DbInterface {
    pub characters: CharacterRepo,
    pub client: PgConnection,
    pub rooms: LocationRepo,
    pub users: UserRepo,
}

impl DbInterface {
    pub fn new(client: PgConnection) -> Self {
        let characters = CharacterRepo {};
        let rooms = LocationRepo {};
        let users = UserRepo {};

        DbInterface {
            characters,
            client,
            rooms,
            users,
        }
    }
}
