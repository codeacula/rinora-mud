use bevy::prelude::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use shared::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::planes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbPlane {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl DbPlane {
    pub fn to_plane(&self) -> Plane {
        Plane {
            description: self.description.clone(),
            id: self.id,
            name: self.name.clone(),
            continents: Vec::new(),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::continents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbContinent {
    pub id: i32,
    pub plane_id: i32,
    pub name: String,
    pub description: String,
}

impl DbContinent {
    pub fn to_continent(&self) -> Continent {
        Continent {
            id: self.id,
            plane_id: self.plane_id,
            name: self.name.clone(),
            description: self.description.clone(),
            areas: Vec::new(),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::areas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbArea {
    pub id: i32,
    pub continent_id: i32,
    pub name: String,
    pub description: String,
}

impl DbArea {
    pub fn to_area(&self) -> Area {
        Area {
            continent_id: self.continent_id,
            description: self.description.clone(),
            id: self.id,
            name: self.name.clone(),
            rooms: Vec::new(),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::environments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbEnvironment {
    pub id: i32,
    pub name: String,
}

impl DbEnvironment {
    pub fn to_environment(&self) -> Environment {
        Environment {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

#[derive(Queryable, Selectable, Default)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbRoom {
    pub id: i32,
    pub area_id: i32,
    pub name: String,
    pub description: String,
    pub environment_id: i32,
}

impl DbRoom {
    pub fn to_room(&self) -> Room {
        Room {
            area_id: self.area_id,
            description: self.description.clone(),
            environment_id: self.environment_id,
            id: self.id,
            name: self.name.clone(),
            exits: Vec::new(),
            entities: Vec::new(),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::exits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbExit {
    pub id: i32,
    pub from_room_id: i32,
    pub to_room_id: i32,
    pub direction: String,
    pub hidden: bool,
}

impl DbExit {
    pub fn to_exit(&self) -> Exit {
        Exit {
            direction: self.direction.clone(),
            from_room_id: self.from_room_id,
            hidden: self.hidden,
            id: self.id,
            to_room_id: self.to_room_id,
            to_room: Entity::PLACEHOLDER,
        }
    }
}

pub struct LocationRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl LocationRepo {
    pub fn new(provided_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        LocationRepo {
            pool: provided_pool,
        }
    }

    /// Convenience method to get a connection
    fn conn(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.get().unwrap()
    }

    pub fn get_all_areas(&self) -> Result<Vec<Area>, String> {
        use crate::schema::areas::dsl::*;

        Ok(areas
            .select(DbArea::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the areas")
            .iter()
            .map(|x| x.to_area())
            .collect::<Vec<Area>>())
    }

    pub fn get_all_continents(&self) -> Result<Vec<Continent>, String> {
        use crate::schema::continents::dsl::*;

        Ok(continents
            .select(DbContinent::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the continents")
            .iter()
            .map(|x| x.to_continent())
            .collect::<Vec<Continent>>())
    }

    pub fn get_all_environments(&self) -> Result<Vec<Environment>, String> {
        use crate::schema::environments::dsl::*;

        Ok(environments
            .select(DbEnvironment::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the environments")
            .iter()
            .map(|x| x.to_environment())
            .collect::<Vec<Environment>>())
    }

    pub fn get_all_exits(&self) -> Result<Vec<Exit>, String> {
        use crate::schema::exits::dsl::*;

        Ok(exits
            .select(DbExit::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the exits")
            .iter()
            .map(|x| x.to_exit())
            .collect::<Vec<Exit>>())
    }

    pub fn get_all_planes(&self) -> Result<Vec<Plane>, String> {
        use crate::schema::planes::dsl::*;

        Ok(planes
            .select(DbPlane::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the planes")
            .iter()
            .map(|x| x.to_plane())
            .collect::<Vec<Plane>>())
    }

    pub fn get_all_rooms(&self) -> Result<Vec<Room>, String> {
        use crate::schema::rooms::dsl::*;

        Ok(rooms
            .select(DbRoom::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the rooms")
            .iter()
            .map(|x| x.to_room())
            .collect::<Vec<Room>>())
    }
}
