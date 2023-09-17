use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Collection,
    sync::Database,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use shared::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password_hash: String,

    #[serde(default)]
    pub autologin: String,
}

pub struct UserRepo {
    pub users: Collection<DbUser>,
}

impl UserRepo {
    pub fn new(database: &Database) -> Self {
        let users = database.collection::<DbUser>("users");

        UserRepo { users }
    }

    /// Given a username and password, creates a new user in the database, returning
    /// the UUID as a String on success, or an Error otherwise
    pub fn create_user(&self, username: &str, password: &str) -> Result<String, String> {
        let password_hash = Sha512::digest(password);

        let new_username = username.to_owned().to_lowercase();

        let new_user = DbUser {
            id: None,
            autologin: "".to_string(),
            username: new_username,
            password_hash: format!("{:x}", password_hash),
        };

        match self.users.insert_one(new_user, None) {
            Ok(res) => Ok(res.inserted_id.as_object_id().unwrap().to_string()),
            Err(e) => Err(format!("Unable to create user: {:?}", e)),
        }
    }

    /// Given a username, returns whether or not the user exists
    pub fn does_user_exist(&self, username: &str) -> Result<bool, String> {
        let query = self
            .users
            .find_one(doc! { "username": username.to_lowercase() }, None);

        if let Err(query_err) = query {
            return Err(format!(
                "Error trying to see if user exists: {:?}",
                query_err
            ));
        }

        let found_user = query.unwrap();

        match found_user {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    /// Gets a User by their Username, or None if none was found
    pub fn find_with_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<User>, String> {
        let password_hash = Sha512::digest(password);

        let query = self.users.find_one(
            doc! { "username": username.to_lowercase(), "password_hash": format!("{:x}", password_hash) },
            None,
        );

        if let Err(query_err) = query {
            return Err(format!(
                "Error trying to find user with credentials: {:?}",
                query_err
            ));
        }

        let found_user = query.unwrap();

        match found_user {
            None => Ok(None),
            Some(user) => Ok(Some(User {
                autologin: user.autologin,
                username: user.username,
                id: user.id.unwrap().to_string(),
            })),
        }
    }

    pub fn get_by_uuid(&self, uuid: &str) -> Result<Option<User>, String> {
        let parse_uuid_result = ObjectId::parse_str(uuid);

        let parsed_uuid = match parse_uuid_result {
            Ok(the_uuid) => the_uuid,
            Err(e) => {
                return Err(format!("Error trying to find user by uuid: {:?}", e));
            }
        };

        let query = self.users.find_one(doc! { "_id": parsed_uuid }, None);

        if let Err(query_err) = query {
            return Err(format!(
                "Error trying to find user by uuid: {:?}",
                query_err
            ));
        }

        let found_user = query.unwrap();

        match found_user {
            None => Ok(None),
            Some(user) => Ok(Some(User {
                autologin: user.autologin,
                username: user.username,
                id: user.id.unwrap().to_string(),
            })),
        }
    }
}
