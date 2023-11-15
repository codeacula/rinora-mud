use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use shared::prelude::*;

use crate::schema::characters;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbCharacter {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub current_room_id: i32,
    pub current_hp: i32,
    pub current_mp: i32,
    pub pronouns: i16,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::characters)]
pub struct NewDbCharacter {
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub current_room_id: i32,
}

impl DbCharacter {
    pub fn to_game_character(&self) -> CharacterBundle {
        CharacterBundle {
            being: Being {
                pronouns: Pronouns(self.pronouns),
            },
            description: Description(self.description.clone()),
            display_name: DisplayName(self.name.clone()),
            health: Health {
                current: self.current_hp,
                max: 0,
            },
            mana: Mana {
                current: self.current_mp,
                max: 0,
            },
            info: Character {
                character_id: self.id,
                user_id: self.user_id,
            },
            location: Location {
                entity: Entity::PLACEHOLDER,
                location_id: self.current_room_id,
            },
        }
    }
}

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
        current_room: i32,
        user: &User,
    ) -> Result<CharacterBundle, String> {
        let name = clean_character_name(charactername);

        let new_character = NewDbCharacter {
            name,
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

    pub fn does_user_own_character(&self, character_name: &str, provided_user_id: i32) -> bool {
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
}
