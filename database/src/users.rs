use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use sha2::{Digest, Sha512};
use shared::prelude::*;

use crate::{prelude::DbInterface, schema::*};

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
    let cleaned_name = provided_username.to_lowercase();

    cleaned_name
}

pub struct UserRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepo {
    pub fn new(provided_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        UserRepo {
            pool: provided_pool,
        }
    }

    fn conn(&self) -> &mut PooledConnection<ConnectionManager<diesel::PgConnection>> {
        &mut self.pool.get().unwrap()
    }

    /// Given a username and password, creates a new user in the database, returning
    /// the new user on success, or an Error
    pub fn create_user(
        &self,
        provided_username: &str,
        provided_password: &str,
    ) -> Result<User, String> {
        let password_hash = Sha512::digest(provided_password);
        let new_username = clean_username(&provided_username);

        let new_user = NewUser {
            username: new_username,
            password_hash: format!("{:x}", password_hash),
        };

        let inserted_user = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(DbUser::as_returning())
            .get_result(self.conn())
            .expect("Error during character creation");

        Ok(inserted_user.to_game_user())
    }

    /// Given a username, returns whether or not the user exists
    pub fn does_user_exist(
        &self,
        conn: &mut PgConnection,
        provided_username: &str,
    ) -> Result<bool, String> {
        use crate::schema::users::dsl::*;
        let result = users
            .select(DbUser::as_select())
            .filter(username.eq(provided_username));

        Ok(result.len() == 1)
    }

    /// Gets a User by their Username, or None if none was found
    pub fn find_with_credentials(
        &self,
        provided_username: &str,
        provided_password: &str,
    ) -> Result<Option<User>, String> {
        todo!("Todo");
        /*
        use crate::schema::users::dsl::*;

        let calculated_hash = Sha512::digest(provided_password);

        let query = users
            .select(DbUser::as_select())
            .filter(username.eq(clean_username(provided_username)));

        let step2 = query.filter(password_hash.eq(calculated_hash));

        Ok(Some(User {
            administrator: false,
            autologin: None,
            id: 0,
            username: "[fff".to_string(),
        }))
        */
    }

    /// Fetch a user by its UUID
    pub fn get_by_id(&self, id: i32) -> Result<Option<User>, String> {
        todo!("Todo");
        // let parse_uuid_result = ObjectId::parse_str(uuid);

        // let parsed_uuid = match parse_uuid_result {
        //     Ok(the_uuid) => the_uuid,
        //     Err(e) => {
        //         return Err(format!("Error trying to find user by uuid: {:?}", e));
        //     }
        // };

        // let query = self.users.find_one(doc! { "_id": parsed_uuid }, None);

        // if let Err(query_err) = query {
        //     return Err(format!(
        //         "Error trying to find user by uuid: {:?}",
        //         query_err
        //     ));
        // }

        // let found_user = query.unwrap();

        // match found_user {
        //     None => Ok(None),
        //     Some(user) => Ok(Some(user.to_game_user())),
        // }
    }
}
