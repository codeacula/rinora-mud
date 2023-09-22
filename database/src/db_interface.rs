use bevy::prelude::*;
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
    pub client: Client,
    pub database: Database,
    pub rooms: RoomRepo,
    pub users: UserRepo,
    pub system: SystemRepo,
}

impl DbInterface {
    pub fn new(host_string: &str, database_name: &str) -> Self {
        let mut client_options = ClientOptions::parse(host_string).unwrap();

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();
        let database = client.database(database_name);

        let characters = CharacterRepo::new(&database);
        let rooms = RoomRepo::new(&database);
        let system = SystemRepo::new(&database);
        let users = UserRepo::new(&database);

        DbInterface {
            characters,
            client,
            database,
            rooms,
            system,
            users,
        }
    }

    pub fn disconnect(self) {
        self.client.shutdown();
    }

    pub fn migrate(&self) -> Result<(), String> {
        let system_info = self.system.get_system_config().unwrap();
        apply_migrations(system_info, self)
    }

    pub fn ping(&self) -> Result<(), Error> {
        match self.database.run_command(doc! {"ping": 1}, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
