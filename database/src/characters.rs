use mongodb::{bson::doc, bson::Uuid, sync::Collection, sync::Database};
use serde::{Deserialize, Serialize};
use shared::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbCharacter {
    pub _id: Uuid,
    pub user_id: String,
    pub name: String,
}

pub struct CharacterRepo {
    pub characters: Collection<DbCharacter>,
}

impl CharacterRepo {
    pub fn new(database: &Database) -> Self {
        let characters = database.collection::<DbCharacter>("characters");

        CharacterRepo { characters }
    }

    pub fn get_all_by_user(&self, user_uuid: String) -> Result<Vec<Character>, String> {
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

            all_chars.push(Character { name: db_char.name });
        }

        Ok(all_chars)
    }
}
