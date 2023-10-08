use bevy::prelude::*;
use bevy::utils::HashMap;
use diesel::{Connection, PgConnection};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use shared::world::*;
use std::env;

use crate::db_interface::DbInterface;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

mod characters;
mod db_interface;
mod locations;
mod schema;
mod settings;
mod users;

pub struct DatabasePlugin;

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(String::from(default))
}

fn add_planes_to_world(db_repo: Res<DbInterface>, mut commands: Commands) {
    let planes = db_repo
        .locations
        .get_all_planes()
        .expect("Unable to fetch planes");

    commands.spawn_batch(planes.into_iter());
}

fn add_continents_to_world(
    db_repo: Res<DbInterface>,
    mut query: Query<&mut Plane>,
    mut commands: Commands,
) {
    let items_to_add = db_repo
        .locations
        .get_all_continents()
        .expect("Unable to fetch all continents");

    let mut item_map: HashMap<i32, Vec<Entity>> = HashMap::new();

    for item in items_to_add {
        let id = item.id;

        if !item_map.contains_key(&id) {
            item_map.insert(id, Vec::new());
        }

        let entity = commands.spawn(item);
        item_map.get_mut(&id).unwrap().push(entity.id());
    }

    for mut parent in query.iter_mut() {
        if item_map.contains_key(&parent.id) {
            parent.continents = item_map.remove(&parent.id).unwrap();
        }
    }
}

fn add_areas_to_world(
    db_repo: Res<DbInterface>,
    mut query: Query<&mut Continent>,
    mut commands: Commands,
) {
    let items_to_add = db_repo
        .locations
        .get_all_areas()
        .expect("Unable to fetch all areas");

    let mut item_map: HashMap<i32, Vec<Entity>> = HashMap::new();

    for item in items_to_add {
        let id = item.id;

        if !item_map.contains_key(&id) {
            item_map.insert(id, Vec::new());
        }

        let entity = commands.spawn(item);
        item_map.get_mut(&id).unwrap().push(entity.id());
    }

    for mut parent in query.iter_mut() {
        if item_map.contains_key(&parent.id) {
            parent.areas = item_map.remove(&parent.id).unwrap();
        }
    }
}

fn add_environments_to_world(db_repo: Res<DbInterface>, mut commands: Commands) {
    let planes = db_repo
        .locations
        .get_all_environments()
        .expect("Unable to fetch environments");

    commands.spawn_batch(planes.into_iter());
}

fn add_rooms_to_world(
    db_repo: Res<DbInterface>,
    mut query: Query<&mut Area>,
    mut commands: Commands,
) {
    let items_to_add = db_repo
        .locations
        .get_all_rooms()
        .expect("Unable to fetch all rooms");

    let mut item_map: HashMap<i32, Vec<Entity>> = HashMap::new();

    for item in items_to_add {
        let id = item.id;

        if !item_map.contains_key(&id) {
            item_map.insert(id, Vec::new());
        }

        let entity = commands.spawn(item);
        item_map.get_mut(&id).unwrap().push(entity.id());
    }

    for mut parent in query.iter_mut() {
        if item_map.contains_key(&parent.id) {
            parent.rooms = item_map.remove(&parent.id).unwrap();
        }
    }
}

fn add_exits_to_world(
    db_repo: Res<DbInterface>,
    mut query: Query<&mut Room>,
    mut commands: Commands,
) {
    let items_to_add = db_repo
        .locations
        .get_all_exits()
        .expect("Unable to fetch all rooms");

    let mut item_map: HashMap<i32, Vec<Entity>> = HashMap::new();

    for item in items_to_add {
        let id = item.id;

        if !item_map.contains_key(&id) {
            item_map.insert(id, Vec::new());
        }

        let entity = commands.spawn(item);
        item_map.get_mut(&id).unwrap().push(entity.id());
    }

    for mut parent in query.iter_mut() {
        if item_map.contains_key(&parent.id) {
            parent.exits = item_map.remove(&parent.id).unwrap();
        }
    }
}

fn add_rooms_to_exits(rooms: Query<(Entity, &Room)>, mut exits: Query<&mut Exit>) {
    // Index all the rooms by id
    let mut room_map: HashMap<i32, Entity> = HashMap::new();

    for (entity, room) in rooms.iter() {
        room_map.insert(room.id, entity);
    }

    for mut exit in exits.iter_mut() {
        exit.to_room = room_map
            .get(&exit.to_room_id)
            .expect("Exit points to room that doesn't exist.")
            .clone();
    }
}

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet)]
struct Planes;

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet)]
struct Continents;

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet)]
struct Areas;

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet)]
struct Environments;

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet)]
struct Rooms;

#[derive(Hash, Debug, Eq, Clone, PartialEq, SystemSet)]
struct Exits;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        let host_string = get_env("DB_CONN_STRING", "postgresql://dev:dev@localhost/rinoramud");

        info!("Attempting to connect to database. {}", &host_string);
        let mut pg_conn = PgConnection::establish(&host_string).unwrap();

        info!("Running migrations {}", &host_string);
        pg_conn.run_pending_migrations(MIGRATIONS).unwrap();

        let repo = DbInterface::new(host_string);

        let settings = repo.settings.get_settings().unwrap();

        app.insert_resource(repo)
            .insert_resource(settings)
            .add_systems(
                Startup,
                (
                    add_planes_to_world.in_set(Planes),
                    add_continents_to_world.in_set(Continents).after(Planes),
                    add_areas_to_world.in_set(Areas).after(Continents),
                    add_environments_to_world.in_set(Environments).after(Areas),
                    add_rooms_to_world.in_set(Rooms).after(Environments),
                    add_exits_to_world.in_set(Exits).after(Rooms),
                    add_rooms_to_exits.after(Exits),
                ),
            );
    }
}

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::db_interface::*;
    pub use crate::locations::*;
    pub use crate::settings::*;
    pub use crate::users::*;
}
