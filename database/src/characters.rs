use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use shared::prelude::*;

use crate::schema::characters;

fn clean_character_name(inc_name: &str) -> String {
    to_title_case(inc_name)
}

pub struct CharacterRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl CharacterRepo {
    pub fn new(provided_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        CharacterRepo {
            pool: provided_pool,
        }
    }

    /// Convenience method to get a connection
    fn conn(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.get().unwrap()
    }

    /// Given a character name and a user, creates a new character and returns it
    pub fn create_character(
        &self,
        charactername: &str,
        pronouns: i16,
        current_room: i32,
        user: &User,
    ) -> Result<CharacterBundle, String> {
        let name = clean_character_name(charactername);

        let new_character = NewDbCharacter {
            name,
            pronouns,
            user_id: user.id,
            description: "A vaguely distinguishable humanoid.".to_string(),
            current_room_id: current_room,
        };

        let inserted_character = diesel::insert_into(characters::table)
            .values(&new_character)
            .returning(DbCharacter::as_returning())
            .get_result::<DbCharacter>(&mut self.conn())
            .expect("Error during character creation");

        Ok(inserted_character.to_game_character())
    }

    /// Deletes a character by their character name
    pub fn delete_character(&self, character_name: &str) -> Result<bool, String> {
        use self::characters::dsl::*;

        let cleaned_name = clean_character_name(character_name);

        let res = diesel::delete(characters)
            .filter(name.eq(cleaned_name))
            .execute(&mut self.conn())
            .expect("Error deleting character by name");

        Ok(res != 0)
    }

    /// Checks to see if a character by the provided username already exists
    pub fn does_character_exist(&self, character_name: &str) -> Result<bool, String> {
        use crate::schema::characters::dsl::*;

        let cleaned_name = clean_character_name(character_name);

        let result: Option<i32> = characters
            .select(id)
            .filter(name.eq(cleaned_name))
            .get_result::<i32>(&mut self.conn())
            .optional()
            .expect("Error while checking if a character exists");

        Ok(result.is_some())
    }

    pub fn does_user_own_character(&self, character_name: &str, provided_user_id: &i32) -> bool {
        use crate::schema::characters::dsl::*;

        let result: i64 = characters
            .filter(name.eq(clean_character_name(character_name)))
            .filter(user_id.eq(provided_user_id))
            .count()
            .get_result::<i64>(&mut self.conn())
            .expect("Unable to determine if a user owns a character");

        result == 1
    }

    /// Returns a charater matching the provided character_name if it exists
    pub fn get_character_by_name(
        &self,
        character_name: &str,
    ) -> Result<Option<CharacterBundle>, String> {
        use crate::schema::characters::dsl::*;

        let result: Option<DbCharacter> = characters
            .select(DbCharacter::as_select())
            .filter(name.eq(clean_character_name(character_name)))
            .get_result::<DbCharacter>(&mut self.conn())
            .optional()
            .expect("Unabled to find character by username.");

        match result {
            None => Ok(None),
            Some(found_character) => Ok(Some(found_character.to_game_character())),
        }
    }

    /// Given a user ID, returns all characters
    pub fn get_all_by_user(&self, provided_user_id: i32) -> Result<Vec<CharacterBundle>, String> {
        use crate::schema::characters::dsl::*;

        let result: Vec<DbCharacter> = characters
            .select(DbCharacter::as_select())
            .filter(user_id.eq(provided_user_id))
            .get_results::<DbCharacter>(&mut self.conn())
            .expect("Unable to fetch all characters by user");

        Ok(result
            .iter()
            .map(|character| character.to_game_character())
            .collect())
    }

    /// Convenience method to get a connection
    pub fn start_transaction(&self) {
        self.pool.get().unwrap().begin_test_transaction().unwrap();
    }

    pub fn update_location(&self, character_id: i32, location_id: i32) -> Result<bool, String> {
        use crate::schema::characters::dsl::*;

        let result = diesel::update(characters)
            .filter(id.eq(character_id))
            .set(current_room_id.eq(location_id))
            .execute(&mut self.conn())
            .expect("Unable to update character location");

        Ok(result == 1)
    }
}
