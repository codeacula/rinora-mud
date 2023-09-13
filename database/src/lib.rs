use bevy::prelude::*;
use shared::prelude::*;
use std::env;

use mongodb::{bson::doc, sync::Client};

pub struct DatabasePlugin;

#[derive(Resource)]
pub struct DbInfo {
    pub client: Client,
}

fn get_env(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        _ => default.to_string(),
    }
}

fn get_connection_string() -> String {
    let database_protocol = get_env("MONGODB_PROTOCOL", "mongodb");
    let database_host = get_env("MONGODB_HOST", "localhost");
    let database_port = get_env("MONGODB_PORT", "27017");

    let database_username = get_env("MONGODB_USERNAME", "");
    let database_password = get_env("MONGODB_PASSWORD", "");

    let mut credentials = String::from("");

    if database_username != "" && database_password != "" {
        credentials = format!("{}:{}@", database_username, database_password);
    }

    let host_string = format!(
        "{}://{}{}:{}",
        database_protocol, credentials, database_host, database_port
    );

    host_string
}

fn get_database() -> String {
    let database_name = get_env("MONGODB_DATABASE", "rinoramud");
    database_name
}

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        let host_string = get_connection_string();
        let database_name = get_database();

        let dbinfo: DbInfo = DbInfo {
            client: Client::with_uri_str(host_string).unwrap(),
        };

        info!("Connecting to database...");
        dbinfo
            .client
            .database(&database_name)
            .run_command(doc! {"ping": 1}, None)
            .unwrap();

        app.insert_resource(dbinfo);
    }
}
