use diesel::{
  prelude::*,
  r2d2::{ConnectionManager, Pool, PooledConnection},
};
use shared::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbSettings {
  pub id: i32,
  pub support_email: String,
  pub default_room: i32,
}

impl DbSettings {
  pub fn to_settings(&self) -> Settings {
    Settings {
      default_room: self.default_room,
      support_email: self.support_email.clone(),
    }
  }
}

pub struct SettingsRepo {
  pool: Pool<ConnectionManager<PgConnection>>,
}

impl SettingsRepo {
  // Opted to go ahead an accept a clone of a Pool because they're backed with Arc, so the clone is cheap
  pub fn new(provided_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
      SettingsRepo {
          pool: provided_pool,
      }
  }

  /// Convenience method to get a connection
  fn conn(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
      self.pool.get().unwrap()
  }

  /// Gets the game settings from the DB
  pub fn get_settings(
      &self
  ) -> Result<Settings, String> {
      use crate::schema::settings::dsl::*;
      
      let found_settings = settings.select(DbSettings::as_select()).first(&mut self.conn()).expect("Unable to fetch settings");

      Ok(found_settings.to_settings())
  }
}
