use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, PgConnection};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use shared::prelude::*;
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

pub fn get_connection_pool(conn_string: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(conn_string);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(String::from(default))
}

fn add_planes_to_world(world: &mut World) {
    let mut system_state: SystemState<Res<DbInterface>> = SystemState::new(world);
    let db_repo: Res<DbInterface> = system_state.get_mut(world);

    let planes = db_repo
        .locations
        .get_all_planes()
        .expect("Unable to fetch planes");

    let mut plane_map = PlaneMap(HashMap::new());

    for item in planes.into_iter() {
        let id = item.plane.plane_id;
        let entity = world.spawn(item);
        plane_map.0.insert(id, entity.id());
    }

    world.insert_resource(plane_map);
}

fn add_continents_to_world(world: &mut World) {
    let mut system_state: SystemState<Res<DbInterface>> = SystemState::new(world);
    let db_repo = system_state.get_mut(world);

    let items_to_add = db_repo
        .locations
        .get_all_continents()
        .expect("Unable to fetch all continents");

    let mut item_map = ContinentMap(HashMap::new());

    for item in items_to_add.into_iter() {
        let id = item.continent.continent_id;
        let entity = world.spawn(item);
        item_map.0.insert(id, entity.id());
    }

    world.insert_resource(item_map);
}

fn add_continents_to_planes(world: &mut World) {
    let mut system_state: SystemState<(
        Query<(Entity, &mut Continent)>,
        Query<(Entity, &Plane)>,
        Commands,
        Res<PlaneMap>,
    )> = SystemState::new(world);
    let (mut children, parents, mut commands, plane_map) = system_state.get_mut(world);

    // Index all the rooms by id
    let mut child_map: HashMap<i32, Vec<Entity>> = HashMap::new();

    for (entity, mut child) in children.iter_mut() {
        if !child_map.contains_key(&child.plane_id) {
            child_map.insert(child.plane_id, Vec::new());
        }

        child.plane = *plane_map.0.get(&child.plane_id).unwrap();
        child_map.get_mut(&child.plane_id).unwrap().push(entity);
    }

    for (entity, parent) in parents.iter() {
        let col = EntityCollection(child_map.get(&parent.plane_id).unwrap().clone());
        commands.entity(entity).insert(col);
    }

    system_state.apply(world);
}

fn add_areas_to_world(world: &mut World) {
    let mut system_state: SystemState<Res<DbInterface>> = SystemState::new(world);
    let db_repo = system_state.get_mut(world);

    let items_to_add = db_repo
        .locations
        .get_all_areas()
        .expect("Unable to fetch all areas");

    let mut item_map = AreaMap(HashMap::new());

    for item in items_to_add.into_iter() {
        let id = item.area.area_id;
        let entity = world.spawn(item);
        item_map.0.insert(id, entity.id());
    }

    world.insert_resource(item_map);
}

fn add_areas_to_continents(world: &mut World) {
    let mut system_state: SystemState<(
        Query<(Entity, &mut Area)>,
        Query<&mut Continent>,
        Res<ContinentMap>,
    )> = SystemState::new(world);
    let (mut children, mut parents, continent_map) = system_state.get_mut(world);

    // Index all the rooms by id
    let mut child_map: HashMap<i32, Vec<Entity>> = HashMap::new();

    for (entity, mut child) in children.iter_mut() {
        if !child_map.contains_key(&child.continent_id) {
            child_map.insert(child.continent_id, Vec::new());
        }

        child_map.get_mut(&child.continent_id).unwrap().push(entity);
        child.continent = *continent_map.0.get(&child.continent_id).unwrap();
    }

    for mut parent in parents.iter_mut() {
        parent.areas = child_map.get(&parent.continent_id).unwrap().clone();
    }
}

fn add_rooms_to_world(world: &mut World) {
    let mut system_state: SystemState<Res<DbInterface>> = SystemState::new(world);
    let db_repo = system_state.get_mut(world);
    let items_to_add = db_repo
        .locations
        .get_all_rooms()
        .expect("Unable to fetch all rooms");

    let mut item_map = RoomMap(HashMap::new());

    for item in items_to_add.into_iter() {
        let id = item.room.room_id;
        let entity = world.spawn(item);
        item_map.0.insert(id, entity.id());
    }

    world.insert_resource(item_map);
}

fn add_environments_to_rooms(world: &mut World) {
    let mut system_state: SystemState<(Res<DbInterface>, Query<(Entity, &mut Room)>)> =
        SystemState::new(world);
    let (db_repo, query) = system_state.get_mut(world);

    let environments = db_repo
        .locations
        .get_all_environments()
        .expect("Failed to fetch all the environments when tagging rooms");

    let mut environment_map: HashMap<i32, Environment> = HashMap::new();

    for env in environments {
        environment_map.insert(env.environment.environment_id, env.environment);
    }

    let mut inserts: Vec<(Entity, Environment)> = Vec::new();

    for (entity, room) in query.iter() {
        if environment_map.contains_key(&room.environment_id) {
            let env = environment_map.get(&room.environment_id).unwrap();
            inserts.push((entity, env.clone()));
        }
    }

    for (entity, env) in inserts {
        world.entity_mut(entity).insert(env);
    }
}

fn add_rooms_to_areas(world: &mut World) {
    let mut system_state: SystemState<(
        Query<(Entity, &mut Room)>,
        Query<(Entity, &Area)>,
        Res<AreaMap>,
        Commands,
    )> = SystemState::new(world);
    let (mut children, parents, area_map, mut commands) = system_state.get_mut(world);

    // Index all the rooms by id
    let mut child_map: HashMap<i32, Vec<Entity>> = HashMap::new();

    for (entity, mut child) in children.iter_mut() {
        if !child_map.contains_key(&child.area_id) {
            child_map.insert(child.area_id, Vec::new());
        }

        child_map.get_mut(&child.area_id).unwrap().push(entity);
        child.area = *area_map.0.get(&child.area_id).unwrap();
    }

    for (entity, parent) in parents.iter() {
        if child_map.contains_key(&parent.area_id) {
            let res = child_map.get(&parent.area_id).unwrap().clone();
            commands.entity(entity).insert(EntityCollection(res));
        }
    }

    system_state.apply(world);
}

fn add_exits_to_world(world: &mut World) {
    let mut system_state: SystemState<Res<DbInterface>> = SystemState::new(world);
    let db_repo = system_state.get_mut(world);
    let items_to_add = db_repo
        .locations
        .get_all_exits()
        .expect("Unable to fetch all rooms");

    world.spawn_batch(items_to_add);
}

fn add_rooms_to_exits(world: &mut World) {
    let mut system_state: SystemState<(
        Query<(Entity, &Room)>,
        Query<(Entity, &mut Exit)>,
        Commands,
    )> = SystemState::new(world);
    let (rooms, exits, mut commands) = system_state.get_mut(world);

    // Index all the rooms by id
    let mut room_map: HashMap<i32, Entity> = HashMap::new();

    for (entity, room) in rooms.iter() {
        room_map.insert(room.room_id, entity);
    }

    for (entity, exit) in exits.iter() {
        let to_room = *room_map
            .get(&exit.to_room_id)
            .expect("Exit points to room that doesn't exist.");

        commands.entity(entity).insert(ExitTo(to_room));
    }

    system_state.apply(world);
}

fn add_exits_to_rooms(world: &mut World) {
    let mut system_state: SystemState<(Query<(Entity, &mut Exit)>, Res<RoomMap>)> =
        SystemState::new(world);
    let (mut exits, room_map) = system_state.get_mut(world);

    let mut room_to_exits: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (exit_entity, mut exit) in exits.iter_mut() {
        if !room_map.0.contains_key(&exit.from_room_id) {
            continue;
        }

        let from_room_entity = room_map.0.get(&exit.from_room_id).unwrap();
        let to_room_entity = room_map.0.get(&exit.to_room_id).unwrap();

        exit.from_room = *from_room_entity;
        exit.to_room = *to_room_entity;

        if !room_to_exits.contains_key(from_room_entity) {
            room_to_exits.insert(*from_room_entity, Vec::new());
        }

        let collection = room_to_exits.get_mut(from_room_entity).unwrap();
        collection.push(exit_entity);
    }

    for (from_room_entity, exits) in room_to_exits {
        world.entity_mut(from_room_entity).insert(Exits(exits));
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ConnectionSettings {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

pub fn get_db_interface(
    conn_settings: ConnectionSettings,
) -> Result<DbInterface, diesel::ConnectionError> {
    let host_string = format!(
        "postgresql://{}:{}@{}:{}/rinoramud",
        conn_settings.username, conn_settings.password, conn_settings.host, conn_settings.port
    );

    let mut pg_conn = PgConnection::establish(&host_string)?;
    pg_conn.run_pending_migrations(MIGRATIONS).unwrap();

    let pool = get_connection_pool(&host_string);

    Ok(DbInterface::new(pool))
}

pub fn get_test_db_interface() -> DbInterface {
    let host_string = "postgresql://devtest:devtest@localhost:5433/rinoratest";

    let mut pg_conn = PgConnection::establish(host_string).unwrap();
    pg_conn.run_pending_migrations(MIGRATIONS).unwrap();

    let pool = get_connection_pool(host_string);

    DbInterface::new(pool)
}

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        let host_string = get_env("DB_CONN_STRING", "postgresql://dev:dev@localhost/rinoramud");

        info!("Attempting to connect to database: {host_string}");
        let mut pg_conn = PgConnection::establish(&host_string).unwrap();

        info!("Running migrations: {host_string}");
        pg_conn.run_pending_migrations(MIGRATIONS).unwrap();

        let pool = get_connection_pool(&host_string);

        let repo = DbInterface::new(pool);

        let settings = repo.settings.get_settings().unwrap();

        app.insert_resource(repo).insert_resource(settings);

        add_planes_to_world(&mut app.world);

        add_continents_to_world(&mut app.world);
        add_continents_to_planes(&mut app.world);

        add_areas_to_world(&mut app.world);
        add_areas_to_continents(&mut app.world);

        add_rooms_to_world(&mut app.world);
        add_environments_to_rooms(&mut app.world);
        add_rooms_to_areas(&mut app.world);

        add_exits_to_world(&mut app.world);
        add_rooms_to_exits(&mut app.world);
        add_exits_to_rooms(&mut app.world);
    }
}

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::db_interface::*;
    pub use crate::locations::*;
    pub use crate::settings::*;
    pub use crate::users::*;

    pub use crate::ConnectionSettings;
    pub use crate::DatabasePlugin;
}
