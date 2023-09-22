use mongodb::bson::doc;

use crate::prelude::*;

use super::initial_migration::InitialMigration;

pub fn apply_migrations(system_info: DbSystemInfo, db_repo: &DbInterface) -> Result<(), String> {
    let mut start_count: usize = system_info.version.try_into().unwrap();

    let mut migrations: Vec<Box<dyn Migration>> = Vec::new();

    migrations.push(Box::new(InitialMigration {}));

    while start_count < migrations.len() {
        let migration = migrations.get(start_count);

        if let Err(e) = migration.unwrap().apply(db_repo) {
            return Err(format!("Error applying migration {}: {:?}", start_count, e));
        }

        start_count += 1;
    }

    let new_version: i32 = start_count.try_into().unwrap();

    let _ = db_repo
        .database
        .collection::<DbSystemInfo>("system")
        .update_many(doc! {}, doc! { "$set": { "version": new_version } }, None);

    Ok(())
}

pub trait Migration {
    fn apply(&self, db_repo: &DbInterface) -> Result<(), String>;
}
