use mongodb::{bson::doc, bson::Uuid, sync::Collection, sync::Database};
use serde::{Deserialize, Serialize};

pub struct UserRepo {
    pub users: Collection<DbUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

pub struct UserInfo {
    username: String,
}

impl UserRepo {
    pub fn new(database: &Database) -> Self {
        let users = database.collection::<DbUser>("users");

        UserRepo { users }
    }

    pub fn does_user_exist(&self, username: &String) -> Result<bool, String> {
        let query = self.users.find_one(doc! { "username": username }, None);

        if query.is_err() {
            let query_err = query.unwrap_err();
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

    pub fn find_by_username(&self, username: &String) -> Result<Option<UserInfo>, String> {
        let query = self.users.find_one(doc! { "username": username }, None);

        if query.is_err() {
            let query_err = query.unwrap_err();
            return Err(format!(
                "Error trying to find user by username: {:?}",
                query_err
            ));
        }

        let found_user = query.unwrap();

        match found_user {
            None => Ok(None),
            Some(user) => Ok(Some(UserInfo {
                username: user.username,
            })),
        }
    }
}
