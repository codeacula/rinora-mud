use mongodb::{bson::doc, bson::Uuid, sync::Collection, sync::Database};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use shared::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub _id: Uuid,
    pub username: String,
    pub password_hash: String,
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
            _id: Uuid::new(),
            username: new_username,
            password_hash: format!("{:x}", password_hash),
        };

        match self.users.insert_one(new_user, None) {
            Ok(res) => Ok(res.inserted_id.to_string()),
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
                autologin: String::from(""),
                username: user.username,
                dbid: user._id.to_string(),
            })),
        }
    }
}
