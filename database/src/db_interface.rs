use bevy::prelude::*;
use mongodb::{bson::doc, error::Error, sync::Client, sync::Database};

#[derive(Resource)]
pub struct DbInterface {
    pub client: Client,
    pub database: Database,
}

impl DbInterface {
    pub fn new(host_string: String, database_name: String) -> Self {
        let client = Client::with_uri_str(host_string).unwrap();
        let database = client.database(&database_name);

        DbInterface { client, database }
    }

    pub fn ping(&self) -> Result<(), Error> {
        match self.database.run_command(doc! {"ping": 1}, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
