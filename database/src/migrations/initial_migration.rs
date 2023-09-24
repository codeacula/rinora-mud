use bevy::utils::Uuid;

use crate::prelude::{DbExit, DbInterface, DbRoom};

use super::apply_migrations::Migration;

pub struct InitialMigration;

impl Migration for InitialMigration {
    fn apply(&self, db_repo: &DbInterface) -> Result<(), String> {
        add_default_rooms(db_repo)?;
        Ok(())
    }
}

fn get_uuid() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

fn insert_room(room: &mut DbRoom, db_repo: &DbInterface) {
    let res = db_repo.rooms.rooms.insert_one(room.clone(), None).unwrap();
    room.id = res.inserted_id.as_object_id();
}

fn add_default_rooms(db_repo: &DbInterface) -> Result<(), String> {
    let mut west_shoreline = DbRoom {
        description: "You're over on the western shoreline.".to_string(),
        name: "Western Shoreline of the Infinite Mirror".to_string(),
        ..Default::default()
    };
    insert_room(&mut west_shoreline, db_repo);

    let mut shoreline = DbRoom {
        description:
            "Before you, stretching out before you to reaches unknown, is the Infinite Mirror."
                .to_string(),
        name: "Before the Infinite Mirror".to_string(),
        exits: vec![],
        ..Default::default()
    };
    insert_room(&mut shoreline, db_repo);

    Ok(())
}
