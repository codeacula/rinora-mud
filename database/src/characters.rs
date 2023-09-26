use diesel::prelude::*;
use shared::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbCharacter {
    pub id: i32,
    pub user_id: i32,
    pub shortname: String,
    pub description: String,
    pub current_room_id: i32,
}

impl DbCharacter {
    pub fn to_game_character(&self) -> Character {
        Character {
            id: self.id,
            shortname: self.shortname.clone(),
            user_id: self.user_id,
            current_room_id: 0,
        }
    }
}

pub struct CharacterRepo;

impl CharacterRepo {
    pub fn create_character(
        &self,
        conn: &PgConnection,
        charactername: &str,
        user: &User,
    ) -> Result<String, String> {
        todo!("Rewrite this!");

        /*
        let name = to_title_case(charactername);

        let new_character = DbCharacter {
            name,
            id: None,
            user_id: user.id.clone(),
        };

        match self.characters.insert_one(new_character, None) {
            Ok(res) => {
                let new_id = res.inserted_id.clone().as_object_id().unwrap();
                Ok(new_id.to_string())
            }
            Err(e) => Err(format!("Unable to create user: {:?}", e)),
        }
        */
    }

    pub fn delete_character(
        &self,
        conn: &PgConnection,
        character_name: &str,
    ) -> Result<bool, String> {
        todo!("Rewrite this!");
        /*
        let res = self
            .characters
            .delete_one(doc! { "name": to_title_case(character_name) }, None);

        if let Err(query_err) = res {
            return Err(format!("Error trying to delete character: {:?}", query_err));
        }

        let delete_result = res.unwrap();

        Ok(delete_result.deleted_count == 1)
        */
    }

    pub fn does_character_exist(
        &self,
        conn: &PgConnection,
        character_name: &str,
    ) -> Result<bool, String> {
        todo!("Rewrite this!");
        /*
        let query_res = self
            .characters
            .find_one(doc! { "name": to_title_case(character_name) }, None);

        if let Err(query_err) = query_res {
            return Err(format!(
                "Error checking if character exists: {:?}",
                query_err
            ));
        }

        Ok(query_res.unwrap().is_some())
        */
    }

    pub fn get_character_by_name(
        &self,
        conn: &PgConnection,
        character_name: &str,
    ) -> Result<Option<Character>, String> {
        todo!("Rewrite this!");

        // let query = self
        //     .characters
        //     .find_one(doc! { "name": to_title_case(character_name) }, None);

        // if let Err(query_err) = query {
        //     return Err(format!("Error trying to get character: {:?}", query_err));
        // }

        // let found_character = query.unwrap();

        // match found_character {
        //     None => Ok(None),
        //     Some(character) => Ok(Some(character.to_game_character())),
        // }
    }

    pub fn get_all_by_user(
        &self,
        conn: &PgConnection,
        user_id: i32,
    ) -> Result<Vec<Character>, String> {
        todo!("Rewrite this!");

        // let query = self.characters.find(doc! { "user_id": user_uuid }, None);

        // if let Err(query_err) = query {
        //     return Err(format!(
        //         "Error getting all characters by user: {:?}",
        //         query_err
        //     ));
        // }

        // let res = query.unwrap();
        // let mut all_chars: Vec<Character> = Vec::new();

        // for db_char_res in res {
        //     let db_char = db_char_res.unwrap();

        //     all_chars.push(db_char.to_game_character());
        // }

        // Ok(all_chars)
    }
}
