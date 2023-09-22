use crate::prelude::{DbInterface, DbRoom};

use super::apply_migrations::Migration;

pub struct InitialMigration;

impl Migration for InitialMigration {
    fn apply(&self, db_repo: &DbInterface) -> Result<(), String> {
        add_default_rooms(db_repo)?;
        Ok(())
    }
}

fn add_default_rooms(db_repo: &DbInterface) -> Result<(), String> {
    let shoreline = DbRoom {
        description: "You sit along a golden shoreline. Ahh".to_string(),
        id: None,
        name: "Along a golden shoreline".to_string(),
        exits: Vec::new(),
    };

    let _ = db_repo.rooms.rooms.insert_one(shoreline, None).unwrap();

    Ok(())
}
