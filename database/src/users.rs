use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use sha2::{Digest, Sha512};
use shared::prelude::*;

use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub autologin: Option<i32>,
    pub administrator: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}

impl DbUser {
    pub fn to_game_user(&self) -> User {
        User {
            administrator: self.administrator,
            autologin: self.autologin,
            username: self.username.clone(),
            id: self.id,
        }
    }
}

pub fn clean_username(provided_username: &str) -> String {
    provided_username.to_lowercase()
}

pub struct UserRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepo {
    // Opted to go ahead an accept a clone of a Pool because they're backed with Arc, so the clone is cheap
    pub fn new(provided_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        UserRepo {
            pool: provided_pool,
        }
    }

    /// Convenience method to get a connection
    fn conn(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.get().unwrap()
    }

    /// Given a username and password, creates a new user in the database, returning
    /// the new user on success, or an Error
    pub fn create_user(
        &self,
        provided_username: &str,
        provided_password: &str,
    ) -> Result<User, String> {
        let password_hash = Sha512::digest(provided_password);
        let new_username = clean_username(provided_username);

        let new_user = NewUser {
            username: new_username,
            password_hash: format!("{:x}", password_hash),
        };

        let inserted_user = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(DbUser::as_returning())
            .get_result::<DbUser>(&mut self.conn())
            .expect("Error during character creation");

        Ok(inserted_user.to_game_user())
    }

    /// Given a username, returns whether or not the user exists
    pub fn does_user_exist(&self, provided_username: &str) -> Result<bool, String> {
        use crate::schema::users::dsl::*;

        let result: Option<i32> = users
            .select(id)
            .filter(username.eq(provided_username))
            .get_result::<i32>(&mut self.conn())
            .optional()
            .expect("Error while checking if a user exists");

        Ok(result.is_some())
    }

    /// Gets a User by their username and password, returning None if not found
    pub fn find_with_credentials(
        &self,
        provided_username: &str,
        provided_password: &str,
    ) -> Result<Option<User>, String> {
        use crate::schema::users::dsl::*;

        let calculated_hash = format!("{:x}", Sha512::digest(provided_password));

        let result: Option<DbUser> = users
            .select(DbUser::as_select())
            .filter(username.eq(clean_username(provided_username)))
            .filter(password_hash.eq(calculated_hash))
            .get_result::<DbUser>(&mut self.conn())
            .optional()
            .expect("Unabled to find user with credentials.");

        match result {
            None => Ok(None),
            Some(found_user) => Ok(Some(found_user.to_game_user())),
        }
    }

    /// Fetch a user by its ID
    pub fn get_by_id(&self, provided_id: i32) -> Result<Option<User>, String> {
        use crate::schema::users::dsl::*;

        let result: Option<DbUser> = users
            .select(DbUser::as_select())
            .filter(id.eq(provided_id))
            .get_result::<DbUser>(&mut self.conn())
            .optional()
            .expect("Unabled to find user by ID.");

        match result {
            None => Ok(None),
            Some(found_user) => Ok(Some(found_user.to_game_user())),
        }
    }
}
