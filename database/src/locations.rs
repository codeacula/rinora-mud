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
    pub fn to_plane(&self) -> PlaneBundle {
        PlaneBundle {
            plane: Plane { plane_id: self.id },
            continents: EntityCollection(Vec::new()),
            description: Description(self.description.clone()),
            name: DisplayName(self.name.clone()),
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
    pub fn to_continent(&self) -> ContinentBundle {
        ContinentBundle {
            continent: Continent {
                continent_id: self.id,
                plane_id: self.plane_id,
                areas: Vec::new(),
            },
            areas: EntityCollection(Vec::new()),
            description: Description(self.description.clone()),
            name: DisplayName(self.name.clone()),
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
    pub fn to_area(&self) -> AreaBundle {
        AreaBundle {
            area: Area {
                continent_id: self.continent_id,
                area_id: self.id,
            },
            description: Description(self.description.clone()),
            name: DisplayName(self.name.clone()),
            rooms: EntityCollection(Vec::new()),
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
    pub fn to_environment(&self) -> EnvironmentBundle {
        EnvironmentBundle {
            environment: Environment {
                environment_id: self.id,
            },
            name: DisplayName(self.name.clone()),
            rooms: EntityCollection(Vec::new()),
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
    pub fn to_room(&self) -> RoomBundle {
        RoomBundle {
            room: Room {
                area_id: self.area_id,
                environment_id: self.environment_id,
                room_id: self.id,
            },
            description: Description(self.description.clone()),
            entities: EntityCollection(Vec::new()),
            exits: Exits(Vec::new()),
            name: DisplayName(self.name.clone()),
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
    pub fn to_exit(&self) -> ExitBundle {
        ExitBundle {
            exit: Exit {
                direction: self.direction.clone(),
                from_room_id: self.from_room_id,
                from_room: Entity::PLACEHOLDER,
                exit_id: self.id,
                to_room: Entity::PLACEHOLDER,
                to_room_id: self.to_room_id,
            },
            to: ExitTo(Entity::PLACEHOLDER),
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

    pub fn get_all_areas(&self) -> Result<Vec<AreaBundle>, String> {
        use crate::schema::areas::dsl::*;

        Ok(areas
            .select(DbArea::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the areas")
            .iter()
            .map(|x| x.to_area())
            .collect())
    }

    pub fn get_all_continents(&self) -> Result<Vec<ContinentBundle>, String> {
        use crate::schema::continents::dsl::*;

        Ok(continents
            .select(DbContinent::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the continents")
            .iter()
            .map(|x| x.to_continent())
            .collect())
    }

    pub fn get_all_environments(&self) -> Result<Vec<EnvironmentBundle>, String> {
        use crate::schema::environments::dsl::*;

        Ok(environments
            .select(DbEnvironment::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the environments")
            .iter()
            .map(|x| x.to_environment())
            .collect())
    }

    pub fn get_all_exits(&self) -> Result<Vec<ExitBundle>, String> {
        use crate::schema::exits::dsl::*;

        Ok(exits
            .select(DbExit::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the exits")
            .iter()
            .map(|x| x.to_exit())
            .collect())
    }

    pub fn get_all_planes(&self) -> Result<Vec<PlaneBundle>, String> {
        use crate::schema::planes::dsl::*;

        Ok(planes
            .select(DbPlane::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the planes")
            .iter()
            .map(|x| x.to_plane())
            .collect())
    }

    pub fn get_all_rooms(&self) -> Result<Vec<RoomBundle>, String> {
        use crate::schema::rooms::dsl::*;

        Ok(rooms
            .select(DbRoom::as_select())
            .get_results(&mut self.conn())
            .expect("Error querying for all the rooms")
            .iter()
            .map(|x| x.to_room())
            .collect())
    }
}
