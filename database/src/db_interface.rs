use bevy::prelude::*;
use mongodb::{
    bson::doc,
    error::Error,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
    sync::Database,
};

use crate::prelude::*;

#[derive(Resource)]
pub struct DbInterface {
    pub characters: CharacterRepo,
    client: Client,
    database: Database,
    pub users: UserRepo,
}

impl DbInterface {
    pub fn new(host_string: &str, database_name: &str) -> Self {
        let mut client_options = ClientOptions::parse(host_string).unwrap();

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();
        let database = client.database(database_name);

        let characters = CharacterRepo::new(&database);
        let users = UserRepo::new(&database);

        DbInterface {
            characters,
            client,
            database,
            users,
        }
    }

    pub fn disconnect(self) {
        self.client.shutdown();
    }

    pub fn ping(&self) -> Result<(), Error> {
        match self.database.run_command(doc! {"ping": 1}, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
