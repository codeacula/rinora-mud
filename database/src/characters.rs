use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Collection,
    sync::Database,
};
use serde::{Deserialize, Serialize};
use shared::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbCharacter {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub name: String,
}

impl DbCharacter {
    pub fn to_game_character(&self) -> Character {
        Character {
            id: self.id.unwrap().to_string(),
            name: self.name.clone(),
            scheduled_for_deletion: false,
            user_id: self.user_id.clone(),
        }
    }
}

pub struct CharacterRepo {
    pub characters: Collection<DbCharacter>,
}

impl CharacterRepo {
    pub fn new(database: &Database) -> Self {
        let characters = database.collection::<DbCharacter>("characters");

        CharacterRepo { characters }
    }

    pub fn create_character(&self, charactername: &str, user: &User) -> Result<String, String> {
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
    }

    pub fn delete_character(&self, character_name: &str) -> Result<bool, String> {
        let res = self
            .characters
            .delete_one(doc! { "name": to_title_case(character_name) }, None);

        if let Err(query_err) = res {
            return Err(format!("Error trying to delete character: {:?}", query_err));
        }

        let delete_result = res.unwrap();

        Ok(delete_result.deleted_count == 1)
    }

    pub fn does_character_exist(&self, character_name: &str) -> Result<bool, String> {
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
    }

    pub fn get_character_by_name(&self, character_name: &str) -> Result<Option<Character>, String> {
        let query = self
            .characters
            .find_one(doc! { "name": to_title_case(character_name) }, None);

        if let Err(query_err) = query {
            return Err(format!("Error trying to get character: {:?}", query_err));
        }

        let found_character = query.unwrap();

        match found_character {
            None => Ok(None),
            Some(character) => Ok(Some(character.to_game_character())),
        }
    }

    pub fn get_all_by_user(&self, user_uuid: &str) -> Result<Vec<Character>, String> {
        let query = self.characters.find(doc! { "user_id": user_uuid }, None);

        if let Err(query_err) = query {
            return Err(format!(
                "Error getting all characters by user: {:?}",
                query_err
            ));
        }

        let res = query.unwrap();
        let mut all_chars: Vec<Character> = Vec::new();

        for db_char_res in res {
            let db_char = db_char_res.unwrap();

            all_chars.push(db_char.to_game_character());
        }

        Ok(all_chars)
    }
}
